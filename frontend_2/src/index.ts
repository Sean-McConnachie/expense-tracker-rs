import * as _ from 'lodash';

import { populateExpenses, populateUsers, addExpense } from './render';
import { DATA } from './vars';


window.onload = pageLoad;

export async function pageLoad() {
	// Important stuff
	await DATA.init();

	populateUsers(DATA.users);
	populateExpenses(DATA.expenses)

	// Button event listeners
	const add_expense_btn = document.getElementById("add-expense")
	add_expense_btn?.addEventListener("click", addExpense)
}

