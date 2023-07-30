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
	is_communal: boolean;
	created_at: Date;
	purchased_at: Date;
	user_owes: UserOwes[];
}

export interface Data {
	users: User[];
	categories: Category[];
	expenses: Expense[];
}

export enum OrderBy {
	Amount,
	Date
}

export interface Filter {
	user_ids: number[];
	category_ids: number[];
	min_amount: number;
	max_amount: number;
	min_date: Date;
	max_date: Date;
	order_by: OrderBy;
	order_asc: boolean;
}

