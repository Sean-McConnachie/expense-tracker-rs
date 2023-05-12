import { Expense, format_date } from "~/utils/models";

export function ExpenseRow({ expense }: { Expense }) {
  return (
    <div>
      <div className="flex items-center justify-between">
        <div className="flex items-center">
          <input
            type="checkbox"
            className="form-checkbox h-5 w-5 text-blue-600"
          />
          <div className="ml-2 text-sm">
            <p className="text-gray-900 whitespace-no-wrap">{expense.amount}</p>
            <p className="text-gray-600 whitespace-no-wrap">
              {format_date(expense.purchased_at)}
            </p>

            <p className="text-gray-600 whitespace-no-wrap">{expense.amount}</p>

            <p className="text-gray-600 whitespace-no-wrap">
              {expense.user_id}
            </p>

            <p className="text-gray-600 whitespace-no-wrap">
              {expense.is_communal ? "Y" : "N"}
            </p>

            <p className="text-gray-600 whitespace-no-wrap">
              {expense.user_owes.length}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
