import { ExpenseRow } from "~/components/expense";
import { Expense } from "~/utils/models";

const ex = {
  id: 1,
  user_id: 1,
  category_id: 1,
  amount: 1.0,
  description: "123",
  is_communal: true,
  created_at: new Date("2022-04-25"),
  purchased_at: new Date("2022-04-25"),
  user_owes: [],
} as Expense;

export default function Expenses() {
  return (
    <div>
      <nav className="flex items-center justify-between flex-wrap bg-blue-500 p-6">
        <div className="flex items-center flex-shrink-0 text-white mr-6">
          <span className="font-semibold text-xl tracking-tight">
            Expense Tracker
          </span>
        </div>
        <div className="w-full flex-grow lg:flex lg:items-center lg:w-auto justify-end">
          <div className="relative">
            <button className="peer px-5 py-2 text-white">
              <img src="cog.svg" width={32} height={32} />
            </button>
            <div className="hidden peer-hover:flex hover:flex w-[180px] flex-col bg-white drop-shadow-lg absolute right-0 transform translate-x-4">
              <a className="px-5 py-3 hover:bg-gray-200" href="#">
                About Us
              </a>
              <a className="px-5 py-3 hover:bg-gray-200" href="#">
                Contact Us
              </a>
              <a className="px-5 py-3 hover:bg-gray-200" href="#">
                Privacy Policy
              </a>
            </div>
          </div>
        </div>
      </nav>
      <main>
        <div className="flex">
          <div className="w-4/5 p-6">
            <ExpenseRow expense={ex} />
          </div>
          <div className="w-1/5 p-6">
            <h2 className="text-2xl font-bold mb-4">Latest News</h2>
            <ul className="list-disc list-inside">
              <li className="mb-2">
                <a href="#" className="text-blue-500 hover:underline">
                  Lorem ipsum dolor sit amet
                </a>
              </li>
              <li className="mb-2">
                <a href="#" className="text-blue-500 hover:underline">
                  Consectetur adipiscing elit
                </a>
              </li>
              <li className="mb-2">
                <a href="#" className="text-blue-500 hover:underline">
                  Vivamus auctor faucibus urna
                </a>
              </li>
              <li className="mb-2">
                <a href="#" className="text-blue-500 hover:underline">
                  Nam ultrices aliquam leo
                </a>
              </li>
              <li>
                <a href="#" className="text-blue-500 hover:underline">
                  Vestibulum tristique felis eu risus
                </a>
              </li>
            </ul>
          </div>
        </div>
      </main>
    </div>
  );
}
