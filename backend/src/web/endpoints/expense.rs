use rocket::serde::json::Json;
use rocket::{get, post, State};

use crate::database::expense;

use datatypes::{Expense, Filter};

#[post("/create", format = "json", data = "<expense>")]
pub async fn expenses_create(
    db_pool: &State<sqlx::PgPool>,
    expense: Json<Expense>,
) -> Result<Json<Expense>, std::io::Error> {
    // let expense = expense::insert_expense(db_pool, expense.0)
    //     .await
    //     .map_err(|_e| std::io::Error::new(std::io::ErrorKind::Other, "Failed to create expense"))?;

    let expense = expense::insert_expense(db_pool, expense.0)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to create expense: {}", e.to_string())))?;

    Ok(Json(expense))
}

#[get("/all")]
pub async fn expenses_all(
    db_pool: &State<sqlx::PgPool>,
) -> Result<Json<Vec<Expense>>, std::io::Error> {
    let expenses = expense::get_expenses(db_pool, None)
        .await
        .map_err(|_e| std::io::Error::new(std::io::ErrorKind::Other, "Failed to get expenses"))?;

    Ok(Json(expenses))
}

#[post("/filter", format = "json", data = "<filter>")]
pub async fn expenses_filter(
    db_pool: &State<sqlx::PgPool>,
    filter: Json<Filter>,
) -> Result<Json<Vec<Expense>>, std::io::Error> {
    let expenses = expense::get_expenses(db_pool, Some(filter.0))
        .await
        .map_err(|_e| std::io::Error::new(std::io::ErrorKind::Other, "Failed to delete user"))?;

    Ok(Json(expenses))
}
