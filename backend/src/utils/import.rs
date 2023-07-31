use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use datatypes::{Category, Expense, User, UserOwes};
use crate::database::category::get_categories;
use crate::database::expense::insert_expense;
use crate::database::user::get_users;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_user<'a>(username: &str, users: &'a Vec<User>) -> anyhow::Result<&'a User> {
    for user in users {
        if user.username() == username {
            return Ok(user);
        }
    }
    return Err(anyhow::anyhow!("User not found."));
}

fn get_category<'a>(category_name: &str, categories: &'a Vec<Category>) -> anyhow::Result<&'a Category> {
    for category in categories {
        if category.name() == category_name {
            return Ok(category);
        }
    }
    return Err(anyhow::anyhow!("Category not found."));
}

pub async fn csv(path: &str, db_pool: &sqlx::PgPool) -> anyhow::Result<()> {
    let mut input = String::new();
    let fp = PathBuf::from(path);
    if !fp.exists() {
        return Err(anyhow::anyhow!("File does not exist."));
    }

    let users = get_users(&db_pool).await?;
    println!("=== Users:");
    for user in &users {
        println!(" -id: `{}`\tusername: `{}`", user.id(), user.username());
    }
    println!("Use the following literal names to create expenses using the csv file? [y/n]");
    io::stdin().read_line(&mut input)?;
    if input.trim() != "y" {
        return Ok(());
    }

    let categories = get_categories(&db_pool).await?;
    println!("=== Categories:");
    for category in &categories {
        println!(" -id: `{}`\tname: `{}`", category.id(), category.name());
    }
    println!("Use the following literal names to create expenses using the csv file? [y/n]");
    input.clear();
    io::stdin().read_line(&mut input)?;
    if input.trim() != "y" {
        return Ok(());
    }

    let mut expenses = vec![];
    if let Ok(lines) = read_lines(fp) {
        for line in lines.skip(1) {
            if let Ok(l) = line {
                let l = l.split(',').collect::<Vec<&str>>();
                let username = l[0].clone();
                let amount = l[1].clone().parse::<f64>()?;
                let category = l[2].clone();
                let date = l[3].clone();
                let associated_users = l[5].trim().split('-').collect::<Vec<&str>>();

                let mut associated_users = if associated_users.len() == 1 && associated_users[0].trim().is_empty() {
                    vec![]
                } else {
                    associated_users
                };
                associated_users.push(username);

                let description = l[6].clone();

                let mut user_owes : Vec<UserOwes> = vec![];
                let cost_per_user = amount / (associated_users.len() as f64);
                for user in associated_users {
                    let user = get_user(user, &users)?;
                    user_owes.push(UserOwes::new(user.id(), -1, cost_per_user));
                }
                let expense = Expense::new(
                    get_user(username, &users)?.id(),
                    get_category(category, &categories)?.id(),
                    amount,
                    description.to_string(),
                    chrono::NaiveDate::parse_from_str(date, "%d/%m/%Y")?,
                    user_owes
                );
                expenses.push(expense);
            }
        }
    }

    input.clear();
    println!("The following expenses will be created:");
    for expense in &expenses {
        println!(" -id: `{}`\tamount: `{}`\tcategory: `{}`\tdate: `{}`\tdescription: `{}`", expense.id(), expense.amount(), expense.category_id(), expense.purchased_at(), expense.description());
        for user_owes in expense.user_owes() {
            println!("   -user_id: `{}`\tamount: `{}`", user_owes.user_id(), user_owes.amount());
        }
    }
    println!("Create the above expenses? [y/n]");
    io::stdin().read_line(&mut input)?;
    if input.trim() != "y" {
        return Ok(());
    }

    for e in expenses {
        insert_expense(&db_pool, e).await?;
    }

    println!("Expenses created.");

    Ok(())
}