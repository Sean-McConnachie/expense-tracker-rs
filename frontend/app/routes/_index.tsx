import { Link, V2_MetaFunction } from "@remix-run/react";

export const meta: V2_MetaFunction = () => {
  return [{ title: "Expense Tracker - Home" }];
};

export default function Index() {
  return (
    <div>
      <Link reloadDocument to="expenses">
        Expenses
      </Link>
    </div>
  )
}