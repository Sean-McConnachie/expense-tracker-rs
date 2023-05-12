const date_options: Intl.DateTimeFormatOptions = {
  day: "2-digit",
  month: "2-digit",
  year: "numeric",
};

export function format_date(date: Date) {
  return date.toLocaleDateString("en-US", date_options);
}

export interface Category {
  id: number;
  name: String;
  created_at: Date;
  description: String;
}

export interface User {
  id: number;
  username: String;
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
  description: String;
  is_communal: boolean;

  created_at: Date;
  purchased_at: Date;

  user_owes: UserOwes[];
}

export enum OrderBy {
  Asc,
  Desc,
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
