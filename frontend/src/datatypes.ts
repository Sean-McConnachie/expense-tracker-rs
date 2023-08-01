import { castFieldToDate, castNestedFieldToDate, http } from "./utils";
import { API_URL } from "./vars";

export interface User {
	id: number;
	username: string;
	created_at: Date;
}

export interface Category {
	id: number;
	name: string;
	description: string;
	created_at: Date;
}

export interface UserOwes {
	id: number;
	user_id: number;
	expense_id: number;
	amount: number;
	created_at: Date;
}

export interface Expense {
	id: number;
	user_id: number;
	category_id: number;
	amount: number;
	description: string;
	created_at: Date;
	purchased_at: Date;
	user_owes: UserOwes[];
}

export enum OrderBy {
	Amount = "amount",
	Date = "date",
}

export interface Filter {
	user_ids: number[];
	category_ids: number[];
	min_amount: number;
	max_amount: number;
	min_date: string;
	max_date: string;
	order_by: string;
	order_asc: boolean;
}

// Program data
export class UserData {
	users: User[];
	private currUsrInd?: number;

	constructor() { }

	public async fetchAllUsers() {
		this.users = await http<User[]>(API_URL + "users/all")
		castFieldToDate(this.users, ["created_at"])
	}

	public loggedInUser(): User {
		if (this.currUsrInd != null) {
			return this.users[this.currUsrInd];
		} else {
			return null;
		}
	}

	public setUser(index: number) {
		this.currUsrInd = index;
	}

	public getById(id: number): User {
		for (let i = 0; i < this.users.length; i++) {
			if (this.users[i].id == id) {
				return this.users[i];
			}
		}
		return null;
	}
}

export class CategoryData {
	categories: Category[];

	constructor() {
		this.categories = [];
	}

	public async fetchAllCategories() {
		this.categories = await http<Category[]>(API_URL + "categories/all");
		castFieldToDate(this.categories, ["created_at"]);
	}

	public getById(id: number): Category {
		for (let i = 0; i < this.categories.length; i++) {
			if (this.categories[i].id == id) {
				return this.categories[i];
			}
		}
		return null;
	}
}

export class ExpenseData {
	expenses: Expense[];

	constructor() {
		this.expenses = [];
	}

	public async fetchAllExpenses() {
		this.expenses = await http<Expense[]>(API_URL + "expenses/all")
		castNestedFieldToDate(this.expenses, ["user_owes"], ["created_at", "purchased_at"], ["created_at"])
	}

	public getById(id: number): Expense {
		for (let i = 0; i < this.expenses.length; i++) {
			if (this.expenses[i].id == id) {
				return this.expenses[i];
			}
		}
		return null;
	}
}

export class Data {
	users: UserData;
	categories: CategoryData;
	expenses: ExpenseData;


	constructor() {
		this.users = new UserData();
		this.categories = new CategoryData();
		this.expenses = new ExpenseData();
	}

	public async init() {
		await this.users.fetchAllUsers();
		await this.categories.fetchAllCategories();
		//await this.expenses.fetchAllExpenses();
	}
}
