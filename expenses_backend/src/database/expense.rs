use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use sqlx::Row;

use super::filter;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Expense {
    id: i32,
    user_id: i32,
    category_id: i32,

    amount: f64,
    description: String,
    is_communal: bool,

    created_at: chrono::NaiveDateTime,
    purchased_at: chrono::NaiveDate,

    user_owes: Vec<UserOwes>,
}

impl Expense {
    pub fn new(
        user_id: i32,
        category_id: i32,
        amount: f64,
        description: String,
        is_communal: bool,
        purchased_at: chrono::NaiveDate,
        user_owes: Vec<UserOwes>,
    ) -> Self {
        Self {
            id: -1,
            user_id,
            category_id,
            amount,
            description,
            is_communal,
            purchased_at,
            created_at: chrono::NaiveDateTime::from_timestamp_millis(0).unwrap(),
            user_owes,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn category_id(&self) -> i32 {
        self.category_id
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn is_communal(&self) -> bool {
        self.is_communal
    }

    pub fn created_at(&self) -> &chrono::NaiveDateTime {
        &self.created_at
    }

    pub fn purchased_at(&self) -> &chrono::NaiveDate {
        &self.purchased_at
    }

    pub fn user_owes(&self) -> &Vec<UserOwes> {
        &self.user_owes
    }

    pub fn add_user_owes(&mut self, user_owes: UserOwes) {
        self.user_owes.push(user_owes);
    }

    pub fn extend_user_owes(&mut self, user_owes: Vec<UserOwes>) {
        self.user_owes.extend(user_owes);
    }
}

impl From<sqlx::postgres::PgRow> for Expense {
    fn from(row: sqlx::postgres::PgRow) -> Self {
        let amount = row.get::<rust_decimal::Decimal, _>("amount");
        Self {
            id: row.get("id"),
            user_id: row.get("user_id"),
            category_id: row.get("category_id"),
            amount: amount.to_f64().unwrap(),
            description: row.get("description"),
            is_communal: row.get("is_communal"),
            created_at: row.get("created_at"),
            purchased_at: row.get("purchased_at"),
            user_owes: vec![],
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct UserOwes {
    id: i32,
    user_id: i32,
    expense_id: i32,
    amount: f64,
    created_at: chrono::NaiveDateTime,
}

impl UserOwes {
    pub fn new(user_id: i32, expense_id: i32, amount: f64) -> Self {
        Self {
            id: -1,
            user_id,
            expense_id,
            amount,
            created_at: chrono::NaiveDateTime::from_timestamp_millis(0).unwrap(),
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn expense_id(&self) -> i32 {
        self.expense_id
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn created_at(&self) -> &chrono::NaiveDateTime {
        &self.created_at
    }
}

impl From<sqlx::postgres::PgRow> for UserOwes {
    fn from(row: sqlx::postgres::PgRow) -> Self {
        let amount = row.get::<rust_decimal::Decimal, _>("amount");
        Self {
            id: row.get("id"),
            user_id: row.get("user_id"),
            expense_id: row.get("expense_id"),
            amount: amount.to_f64().unwrap(),
            created_at: row.get("created_at"),
        }
    }
}

pub async fn insert_expense(
    db_pool: &sqlx::PgPool,
    expense: Expense,
) -> Result<Expense, sqlx::Error> {
    let sql = r#"
    INSERT INTO expenses (user_id, category_id, amount, description, is_communal, purchased_at)
    VALUES ($1, $2, $3, $4, $5, $6)
    RETURNING id, user_id, category_id, amount, description, is_communal, purchased_at, created_at
    "#;

    let mut tx = db_pool.begin().await?;

    let row = sqlx::query(sql)
        .bind(expense.user_id())
        .bind(expense.category_id())
        .bind(expense.amount())
        .bind(expense.description())
        .bind(expense.is_communal())
        .bind(expense.purchased_at())
        .fetch_one(&mut tx)
        .await?;
    let mut inserted_expense = Expense::from(row);

    for user in expense.user_owes() {
        let sql = r#"
        INSERT INTO user_owes (user_id, expense_id, amount)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, expense_id, amount, created_at
        "#;

        let row = sqlx::query(sql)
            .bind(user.user_id())
            .bind(inserted_expense.id())
            .bind(user.amount())
            .fetch_one(&mut tx)
            .await?;
        let user_owes = UserOwes::from(row);
        inserted_expense.add_user_owes(user_owes);
    }

    tx.commit().await?;

    Ok(inserted_expense)
}

pub async fn get_user_owes(
    db_pool: &sqlx::PgPool,
    expense_id: i32,
) -> Result<Vec<UserOwes>, sqlx::Error> {
    let sql = r#"
    SELECT id, user_id, expense_id, amount, created_at
    FROM user_owes
    WHERE expense_id = $1
    "#;

    let user_owes = sqlx::query(sql)
        .bind(expense_id)
        .fetch_all(db_pool)
        .await?
        .into_iter()
        .map(|row| UserOwes::from(row))
        .collect::<Vec<UserOwes>>();

    Ok(user_owes)
}

pub async fn get_expenses(
    db_pool: &sqlx::PgPool,
    filter: Option<filter::Filter>,
) -> Result<Vec<Expense>, sqlx::Error> {
    let mut expenses = if filter.is_none() {
        let sql = r#"
        SELECT id, user_id, category_id, amount, description, is_communal, purchased_at, created_at
        FROM expenses
        ORDER BY created_at DESC
        "#;

        let expenses = sqlx::query(sql)
            .fetch_all(db_pool)
            .await?
            .into_iter()
            .map(|row| Expense::from(row))
            .collect::<Vec<Expense>>();
        expenses
    } else {
        let filter = filter.unwrap();
        let sql = format!(
            r#"
        SELECT id, user_id, category_id, amount, description, is_communal, purchased_at, created_at
        FROM expenses
        WHERE user_id = ANY($1)
        AND category_id = ANY($2)
        AND amount >= $3
        AND amount <= $4
        AND purchased_at >= $5
        AND purchased_at <= $6
        ORDER BY {} {}
        "#,
            if filter.order_by() == &filter::OrderBy::Amount {
                "amount"
            } else {
                "purchased_at"
            },
            if filter.order_asc() == true {
                "ASC"
            } else {
                "DESC"
            }
        );

        let expenses = sqlx::query(&sql)
            .bind(filter.user_ids())
            .bind(filter.category_ids())
            .bind(filter.min_amount())
            .bind(filter.max_amount())
            .bind(filter.min_date())
            .bind(filter.max_date())
            .fetch_all(db_pool)
            .await?
            .into_iter()
            .map(|row| Expense::from(row))
            .collect::<Vec<Expense>>();

        expenses
    };
    for expense in expenses.iter_mut() {
        let user_owes = get_user_owes(db_pool, expense.id()).await?;
        expense.extend_user_owes(user_owes);
    }
    return Ok(expenses);
}

#[cfg(test)]
mod test {
    use rust_decimal::prelude::FromPrimitive;

    use super::*;

    #[test]
    fn f64_to_decimal() {
        let x = 1;
        let y = rust_decimal::Decimal::from_i64(x);
        dbg!(x, y);
    }

    #[tokio::test]
    async fn test_insert() {
        let db_config = crate::database::DbConfig {
            database_url: "postgres://postgres:12341234@localhost/expense_tracker".to_string(),
            max_connections: 5,
        };
        let db_pool = crate::database::connect_db(&db_config).await.unwrap();

        let expense = Expense::new(
            5,
            2,
            100.1235,
            "test expense1".to_string(),
            true,
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            vec![UserOwes::new(6, 1, 100.0)],
        );

        let inserted_expense = insert_expense(&db_pool, expense)
            .await
            .expect("failed to insert expense");

        dbg!(&inserted_expense);
    }

    #[tokio::test]
    async fn test_get_expenses() {
        let db_config = crate::database::DbConfig {
            database_url: "postgres://postgres:12341234@localhost/expense_tracker".to_string(),
            max_connections: 5,
        };
        let db_pool = crate::database::connect_db(&db_config).await.unwrap();

        let expenses = get_expenses(&db_pool, None)
            .await
            .expect("failed to get expenses");

        dbg!(&expenses);
    }

    #[tokio::test]
    async fn test_get_expenses_with_filter() {
        let db_config = crate::database::DbConfig {
            database_url: "postgres://postgres:12341234@localhost/expense_tracker".to_string(),
            max_connections: 5,
        };
        let db_pool = crate::database::connect_db(&db_config).await.unwrap();

        let filter = filter::Filter {
            user_ids: vec![5],
            category_ids: vec![1, 2],
            min_amount: 100.0,
            max_amount: 1000.0,
            min_date: chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            max_date: chrono::NaiveDate::from_ymd_opt(2021, 12, 31).unwrap(),
            order_by: filter::OrderBy::Amount,
            order_asc: true,
        };

        let expenses = get_expenses(&db_pool, Some(filter))
            .await
            .expect("failed to get expenses");

        dbg!(&expenses);
    }
}
