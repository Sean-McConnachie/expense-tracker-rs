import * as _ from "lodash";
import * as Handlebars from "handlebars";

import { Category, Expense, ExpenseData, User, UserData } from './datatypes'
import { API_URL, DATA } from "./vars";
import { http } from "./utils";
import { calculateExpenses } from "./calculations";

type success = boolean;

function populateTemplate(template_id: string, target_id: string, data: any): success {
	const raw_template = document.getElementById(template_id);
	if (!raw_template) {
		console.error("Unable to load template: `" + template_id + "`");
		return false as success;
	}
	const compiled_template = Handlebars.compile(raw_template.innerHTML);
	const html_template = compiled_template(data);
	const target = document.getElementById(target_id);
	if (!target) {
		console.error("Unable to load target: `" + target_id + "`");
		return false as success;
	}
	target.innerHTML = html_template;
	return true as success;
}


// ==== User Dropdown
export function populateUsers(user_data: UserData) {
	// Create data format
	interface UserDdData {
		current_user: string;
		users: User[]
	}

	const user_dd_transformed: UserDdData = {
		"current_user": "Select User",
		"users": user_data.users
	};

	{
		const u = user_data.loggedInUser();
		if (u != null) {
			user_dd_transformed.current_user = u.username;
		}
	}

	populateTemplate("user-dd-template", "user-dd-target", user_dd_transformed);

	const user_select_elem = document.getElementById("user-dd-select");
	user_select_elem.addEventListener('change', callbackUserSelect);
}

function callbackUserSelect(event: Event) {
	const e = event.target as HTMLSelectElement;

	const uid = e.value;
	if (uid == null) {
		console.error("Invalid uid")
		return;
	}

	DATA.users.setUser(parseInt(uid));
}


// ===== Expenses
export function populateExpenses(expense_data: ExpenseData) {
	interface UserOwesLi {
		name: string;
		amount: number;
		perc: number;
	}

	interface ExpenseLi {
		checked: boolean;
		amount: number;
		pretty_date: string;
		category_name: string;
		username: string;
		num_users: number;
		user_owes: UserOwesLi[];
		description: string;
	}

	interface ExpenseListData {
		entries: ExpenseLi[]
	}

	const expense_transformed: ExpenseListData = {
		entries: []
	}
	for (let i = 0; i < expense_data.expenses.length; i++) {
		const e = expense_data.expenses[i];

		const category_name = DATA.categories.getById(e.category_id).name;
		const username = DATA.users.getById(e.user_id).username;

		const user_owes: UserOwesLi[] = [];

		for (let j = 0; j < e.user_owes.length; j++) {
			user_owes.push({
				name: DATA.users.getById(e.user_owes[j].user_id).username,
				amount: e.user_owes[j].amount,
				perc: (e.user_owes[j].amount / e.amount) * 100
			})
		}

		const description = ((e.description == "") ? "No description." : e.description);

		expense_transformed.entries.push({
			checked: false,
			amount: e.amount,
			pretty_date: e.purchased_at.toDateString(),
			category_name: category_name,
			username: username,
			num_users: e.user_owes.length,
			user_owes: user_owes,
			description: description
		})
	}

	populateTemplate("expense-template", "expense-target", expense_transformed);

	const checkbox_elems = document.getElementsByClassName("expense-checkbox");
	for (let i = 0; i < checkbox_elems.length; i++) {
		checkbox_elems[i].addEventListener("change", updateExpenseSummary);
	}

	updateExpenseSummary();
}

// ===== Add expense overlay
function showLoginMsg() {
	const show_login_elem = document.getElementById("login-msg-target");
	show_login_elem.classList.remove("hidden");
}

interface AddExpenseData {
	current_date: string;
	current_user: string;
	categories: Category[];
	users: User[];
}

export function addExpense(_: Event) {
	const current_user = DATA.users.loggedInUser();
	if (current_user === null) {
		showLoginMsg();
		return;
	}

	const add_expense_data: AddExpenseData = {
		current_date: new Date().toISOString().slice(0, 10),
		current_user: current_user.username,
		categories: DATA.categories.categories,
		users: DATA.users.users
	};

	populateTemplate("add-expense-template", "add-expense-target", add_expense_data)

	const input_elems = document.getElementsByClassName("user-amount");
	const checkbox_elems = document.getElementsByClassName("user-checked");
	for (let i = 0; i < input_elems.length; i++) {
		input_elems[i].addEventListener("input", updateSum);
		checkbox_elems[i].addEventListener("input", updateSum);
	}
	const total_elem = document.getElementById("total");
	total_elem.addEventListener("input", updateOthers);

	const submit_elem = document.getElementById("submit-expense");
	submit_elem.addEventListener("click", submitExpense);
}

function updateSum(_: Event) {
	let s = 0;
	const input_elems = document.getElementsByClassName("user-amount");
	const checkbox_elems = document.getElementsByClassName("user-checked");
	var amount_inp;
	var check_inp;
	for (let i = 0; i < input_elems.length; i++) {
		amount_inp = input_elems[i] as HTMLInputElement;
		check_inp = checkbox_elems[i] as HTMLInputElement;
		if (check_inp.checked === true) {
			s += parseFloat(amount_inp.value);
		} else {
			amount_inp.value = "0.00"
		}
	}

	const total_elem = document.getElementById("total") as HTMLInputElement;
	total_elem.value = s.toFixed(2);
}


function updateOthers(_: Event) {
	const total_elem = document.getElementById("total") as HTMLInputElement;
	const input_elems = document.getElementsByClassName("user-amount");
	const checkbox_elems = document.getElementsByClassName("user-checked");

	let num_splits = 0;
	var amount_inp, check_inp;
	for (let i = 0; i < input_elems.length; i++) {
		check_inp = checkbox_elems[i] as HTMLInputElement;
		if (check_inp.checked === true) {
			num_splits++;
		}
	}

	const equal_split = (parseFloat(total_elem.value) / num_splits).toFixed(2);

	for (let i = 0; i < input_elems.length; i++) {
		check_inp = checkbox_elems[i] as HTMLInputElement;
		amount_inp = input_elems[i] as HTMLInputElement;
		if (check_inp.checked == true) {
			amount_inp.value = equal_split;
		}
	}
}

function displayError(message: string) {
	const error_elem = document.getElementById("expense-error");
	error_elem.innerText = message;
}

function submitExpense(_: Event) {
	const logged_in_user = DATA.users.loggedInUser();
	if (logged_in_user === null) {
		showLoginMsg();
		return;
	}

	const expense: Expense = {
		user_id: logged_in_user.id,
		created_at: new Date(),
		id: -1,
		category_id: null,
		amount: null,
		description: null,
		purchased_at: null,
		user_owes: []
	}

	{
		const category_elem = document.getElementById("category") as HTMLSelectElement;
		const category_id = category_elem.value;
		if (category_id == "" || category_id == null) {
			displayError("Invalid category!")
			return;
		}
		expense.category_id = DATA.categories.categories[parseInt(category_id)].id;
	}

	{
		const description_elem = document.getElementById("description") as HTMLTextAreaElement;
		expense.description = description_elem.value;
	}

	{
		const date_elem = document.getElementById("date") as HTMLInputElement;
		const date = new Date(date_elem.value)

		if (isNaN(date.getTime())) {
			displayError("Invalid date!");
			return;
		}

		expense.purchased_at = date;
	}

	var total = 0;
	{
		const input_elems = document.getElementsByClassName("user-amount");
		const checkbox_elems = document.getElementsByClassName("user-checked");

		var amount_inp, check_inp;
		var user;
		for (let i = 0; i < input_elems.length; i++) {
			check_inp = checkbox_elems[i] as HTMLInputElement;
			if (check_inp.checked === true) {
				amount_inp = input_elems[i] as HTMLInputElement;
				const amount = parseFloat(amount_inp.value);

				user = DATA.users.users[parseInt(amount_inp.getAttribute("uid"))];

				if (amount < 0) {
					displayError("Amount for `" + user.username + "` is less than $0!");
					return;
				}
				expense.user_owes.push({
					id: -1,
					user_id: user.id,
					expense_id: -1,
					amount: amount,
					created_at: new Date()
				})

				total += amount;
			}
		}
	}

	if (total <= 0.001) {
		displayError("Total amount must be greater than $0.001!");
		return;
	}

	expense.amount = total;

	// ===
	{
		Date.prototype.toJSON = function(): string {
			const year = this.getUTCFullYear();
			const month = String(this.getUTCMonth() + 1).padStart(2, '0');
			const day = String(this.getUTCDate()).padStart(2, '0');
			const hours = String(this.getUTCHours()).padStart(2, '0');
			const minutes = String(this.getUTCMinutes()).padStart(2, '0');
			const seconds = String(this.getUTCSeconds()).padStart(2, '0');

			const formattedDate = `${year}-${month}-${day}T${hours}:${minutes}:${seconds}`;

			return formattedDate;
		};

		http<Expense>(
			API_URL + "expenses/create",
			{
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify(formatExpenseDates(expense))
			}
		)
			.then((new_expense) => {
				if (new_expense != null) {
					// This would be best, however, sorting and logic makes it annoying.
					// Since the server is going to be running locally it is easier to simply make another request
					// for all the data.
					//DATA.expenses.expenses.push(new_expense);
					DATA.expenses.fetchAllExpenses();
					populateExpenses(DATA.expenses);
				}
			});
	}

	const overlay_elem = document.getElementById("add-expense-target");
	overlay_elem.innerHTML = ``;
}

function formatExpenseDates(expense: Expense) {
	return {
		...expense,
		purchased_at: expense.created_at.toISOString().slice(0, 10), // Format "2021-01-01"
	};
}

// ===

export function updateExpenseSummary() {
	const expense_summary = calculateExpenses();
	populateTemplate("expense-summary-template", "expense-summary-target", expense_summary);
}

Handlebars.registerHelper('rnd2DP', function(distance) {
	return distance.toFixed(2);
});
