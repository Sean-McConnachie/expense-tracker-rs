use anyhow::{anyhow, Result};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};

#[derive(PartialEq)]
pub enum InputResult<T> {
    Value(T),
    Return,
}

pub enum Task {
    AddExpense,
    ViewExpenses,
    MakeCalculations,
}

pub fn select_task() -> InputResult<Task> {
    println!("Select task:");
    println!("1> Add expense");
    println!("2> View expenses");
    println!("3> Make calculations");
    println!("0> Exit");

    let selection = parse_int_input(0..=3);
    match selection {
        0 => InputResult::Return,
        1 => InputResult::Value(Task::AddExpense),
        2 => InputResult::Value(Task::ViewExpenses),
        3 => InputResult::Value(Task::MakeCalculations),
        _ => unreachable!(),
    }
}

pub fn select_user(users: &Vec<User>) -> InputResult<&User> {
    println!("Select user:");
    for (i, user) in users.iter().enumerate() {
        println!("{}. {}", i + 1, user.name);
    }
    println!("0> Return");

    let selection = parse_int_input(0..=users.len() as i32);
    match selection {
        0 => InputResult::Return,
        _ => InputResult::Value(&users[selection as usize - 1]),
    }
}

pub fn select_category(categories: &Vec<Category>) -> &Category {
    println!("Select category:");
    for (i, category) in categories.iter().enumerate() {
        println!("{}. {}", i + 1, category.name);
    }

    let selection = parse_int_input(1..=categories.len() as i32);
    &categories[selection as usize - 1]
}

pub fn parse_int_input(range: std::ops::RangeInclusive<i32>) -> i32 {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input.parse::<i32>() {
            Ok(i) => {
                if range.contains(&i) {
                    return i;
                }
            }
            Err(_) => println!("Invalid input! Try again."),
        }
    }
}

pub fn parse_float_input() -> f32 {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input.parse::<f32>() {
            Ok(f) => return f,
            Err(_) => println!("Invalid input! Try again."),
        }
    }
}

pub fn parse_date_input() -> DateTime<Utc> {
    println!("Enter date [dd/mm/yyyy] (empty for today's date):");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match NaiveDate::parse_from_str(input, "%d/%m/%Y") {
            Ok(date) => {
                let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
                let date_time = NaiveDateTime::new(date, midnight);
                let date_time_utc = DateTime::<Utc>::from_utc(date_time, Utc);
                return date_time_utc;
            }
            Err(_) => {
                if input.is_empty() {
                    return Utc::now();
                }
                println!("Invalid input! Try again.");
            }
        }
    }
}

pub fn get_string_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn parse_confirmation(default: bool) -> bool {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input.to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            "" => return default,
            _ => println!("Invalid input! Try again."),
        }
    }
}

pub fn parse_shared_amount(total_amount: f32) -> f32 {
    println!("Enter shared amount ($<amount> | <ratio>%). INCLUDE CHARS!!");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match calculate_shared_amount(total_amount, input) {
            Ok(amount) => {
                if amount > total_amount {
                    println!("Shared amount cannot be greater than total amount!");
                } else {
                    return amount;
                }
            }
            Err(_) => println!("Invalid input! Try again."),
        }
    }
}

fn calculate_shared_amount(total_amount: f32, input: &str) -> Result<f32> {
    if input.starts_with("$") {
        match input[1..].parse::<f32>() {
            Ok(amount) => Ok(amount),
            Err(_) => return Err(anyhow!("Invalid input! Try again.")),
        }
    } else if input.ends_with("%") {
        match input[..input.len() - 1].parse::<f32>() {
            Ok(ratio) => Ok(total_amount * ratio / 100.0),
            Err(_) => Err(anyhow!("Invalid input! Try again.")),
        }
    } else {
        Err(anyhow!("Invalid input! Try again."))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_shared_amount() {
        use super::calculate_shared_amount;
        let total_amount = 30.0 as f32;
        let actual = calculate_shared_amount(total_amount, "$10.0").unwrap();
        let expected = 10.0 as f32;
        assert_eq!(expected, actual);

        let expected = 15.0 as f32;
        let actual = calculate_shared_amount(total_amount, "50.0%").unwrap();
        assert_eq!(expected, actual);

        let actual = calculate_shared_amount(total_amount, "50%").unwrap();
        assert_eq!(expected, actual);

        let actual = calculate_shared_amount(total_amount, "50");
        assert_eq!(true, actual.is_err());
    }
}
