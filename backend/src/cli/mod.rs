mod input;

use crate::categories::Category;
use crate::cli::input::*;
use crate::expenses::FullExpense;
use crate::users::User;
use crate::utils::{config, into_vec};
// use crate::{categories, expenses, users, utils};
use anyhow::Result;
use log::{error, info};

pub async fn run(db_pool: sqlx::PgPool) -> Result<()> {
    let users = into_vec::<User>(users::list(&db).await)?;
    let categories = into_vec::<Category>(categories::list(&db).await)?;
    loop {
        match select_task() {
            InputResult::Return => break,
            InputResult::Value(task) => match task {
                Task::AddExpense => match add_expense(&users, &categories, db).await {
                    Ok(_) => (),
                    Err(e) => error!("Error adding expense: {}", e),
                },
                Task::ViewExpenses => match show_expenses(db).await {
                    Ok(_) => (),
                    Err(e) => error!("Error showing expenses: {}", e),
                },
                Task::MakeCalculations => (),
            },
        }
    }
    Ok(())
}

async fn show_expenses(db: &utils::DB) -> Result<()> {
    let exps = expenses::list_all(&db).await?;
    for (i, expense) in exps.iter().enumerate() {
        println!("Expense #{} ====================================", i + 1);
        println!("{expense}",);
    }
    Ok(())
}

async fn add_expense(usrs: &Vec<User>, cats: &Vec<Category>, db: &utils::DB) -> Result<()> {
    let user = select_user(usrs);
    let user = if let InputResult::Value(user) = user {
        user
    } else {
        return Ok(());
    };

    let category = select_category(cats);

    println!("Enter amount:");
    let total_amount = parse_float_input();

    let purchase_date = parse_date_input();

    println!("Enter description:");
    let description = get_string_input();

    println!("Is communal? [Y/n]:");
    let is_communal = parse_confirmation(true);

    let mut owings: Vec<(f32, User)> = vec![];
    let is_equal_split;
    if is_communal == true {
        println!("Equally split? [Y/n]:");
        is_equal_split = parse_confirmation(true);
        if is_equal_split == true {
            for u in usrs {
                if u == user {
                    continue;
                }
                owings.push((total_amount / usrs.len() as f32, u.clone()))
            }
        } else {
            let mut temp_users = usrs.clone();
            remove_vec_item(&mut temp_users, &user);
            for i in 1..usrs.len() {
                println!("Associated user {i}:");
                match select_user(&temp_users) {
                    InputResult::Value(u) => {
                        let user_amount = parse_shared_amount(total_amount);
                        let ind = find_index(usrs, u);
                        owings.push((user_amount, usrs[ind].clone()));
                        let remove_ind = find_index(&temp_users, u);
                        temp_users.remove(remove_ind);
                    }
                    InputResult::Return => break,
                };
            }
        }
    }

    // output details
    let full_expense = FullExpense::new(
        user.clone(),
        total_amount,
        category.clone(),
        purchase_date,
        is_communal,
        owings,
        description,
    );
    println!("RECORD:");
    println!("{full_expense}");
    println!("Confirm input [Y/n]");

    let confirm_input = parse_confirmation(true);
    match confirm_input {
        true => {
            expenses::create(full_expense, &db).await?;
            info!("Added expense.");
        } // TODO
        false => info!("Cancelled adding expense."),
    }
    Ok(())
}

fn remove_vec_item<T: PartialEq>(vec: &mut Vec<T>, obj: &T) {
    let index = vec.iter().position(|x| x == obj).unwrap();
    vec.remove(index);
}

fn find_index<T: PartialEq>(vec: &Vec<T>, obj: &T) -> usize {
    vec.iter().position(|x| x == obj).unwrap()
}
