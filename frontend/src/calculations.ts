import { round } from "lodash";
import { DATA } from "./vars";

interface WeightedEdges {
	[key: number]: number
}

interface AdjacencyList {
	[key: number]: WeightedEdges;
}

interface Owing {
	user_who_pays: string;
	user_who_receives: string;
	amount: number;
}

interface Spent {
	name: string;
	amount: number;
}

interface ExpenseSummary {
	total: number;
	categories: string[];
	user_map: Owing[];
	totals_spent: Spent[];
}

export function calculateExpenses(): ExpenseSummary {
	const expense_summary: ExpenseSummary = {
		total: 0,
		categories: [],
		totals_spent: [],
		user_map: [] // Add at the end
	}
	const user_map: AdjacencyList = {};

	{
		// Initialize map
		const len = DATA.users.users.length;
		var curr, other;
		for (let i = 0; i < len; i++) {
			curr = DATA.users.users[i];
			user_map[curr.id] = {};

			for (let j = 0; j < len; j++) {
				other = DATA.users.users[j];
				if (curr.id == other.id) { continue; }

				user_map[curr.id][other.id] = 0;
			}
		}
	}

	{
		// Populate map based on checkbox elements
		const checkbox_elems = document.getElementsByClassName("expense-checkbox");
		console.assert(checkbox_elems.length == DATA.expenses.expenses.length);

		var cb;
		var expense;

		const expense_cat_ids: number[] = [];
		const totals_spent: WeightedEdges = [];
		for (let u = 0; u < DATA.users.users.length; u++) {
			totals_spent[DATA.users.users[u].id] = 0;
		}


		for (let i = 0; i < checkbox_elems.length; i++) {
			cb = checkbox_elems[i] as HTMLInputElement;
			if (cb.checked === true) {
				expense = DATA.expenses.expenses[i];

				if (!expense_cat_ids.includes(expense.category_id)) {
					expense_cat_ids.push(expense.category_id);
				}

				expense_summary.total += expense.amount;
				totals_spent[expense.user_id] += expense.amount;

				var owe;
				for (let u = 0; u < expense.user_owes.length; u++) {
					owe = expense.user_owes[u];

					if (owe.user_id == expense.user_id) { continue; }

					if (owe.amount < 0) { continue; }

					user_map[expense.user_id][owe.user_id] += owe.amount;
				}
			}
		}
		// Append literal strings for category names to expense_summary
		for (let i = 0; i < expense_cat_ids.length; i++) {
			expense_summary.categories.push(DATA.categories.getById(expense_cat_ids[i]).name);
		}

		// Append literal strings and amounts to expense_summary
		for (const u in totals_spent) {
			expense_summary.totals_spent.push({
				name: DATA.users.getById(parseInt(u)).username,
				amount: totals_spent[u]
			})
		}
	}

	{
		// Reduce amount owed between each user, resulting in a single directed edge between each person.
		let outneighbours;
		let u_owes_v, v_owes_u;
		for (const u in user_map) {

			outneighbours = user_map[u];
			for (const v in outneighbours) {

				v_owes_u = outneighbours[v];
				u_owes_v = user_map[v][u];

				if (u_owes_v >= v_owes_u) {
					//user_map[u][v] = 0;
					delete user_map[v][u];
					user_map[u][v] = u_owes_v - v_owes_u;
				}
			}
		}
	}

	{
		// Simplify the amounts using an algorithm
		// Optimization level can be 1 if there exists a topological sort of user_map and the traversal is done in that order.
		const optimization_level = round(DATA.users.users.length / 2);
		let amount;
		for (let optimization_round = 0; optimization_round < optimization_level; optimization_round++) {
			for (const u in user_map) {
				for (const v in user_map[u]) {
					for (const w in user_map[v]) {
						if (user_map[u][w] == undefined) { continue; }

						//if (user_map[u][v] < 0 || user_map[v][w] < 0) {
						//continue;
						if (user_map[u][v] < user_map[v][w]) {
							amount = user_map[u][v];
						} else {
							amount = user_map[v][w];
						}

						user_map[v][w] -= amount;
						user_map[u][v] -= amount;
						user_map[u][w] += amount;
					}
				}
			}
		}
	}

	{
		// Add owings to expense_summary
		for (const u in user_map) {
			for (const v in user_map[u]) {
				if (user_map[u][v] != 0) {
					expense_summary.user_map.push({
						user_who_pays: DATA.users.getById(parseInt(u)).username,
						user_who_receives: DATA.users.getById(parseInt(v)).username,
						amount: user_map[u][v]
					})
				}
			}
		}
	}

	return expense_summary;
}
