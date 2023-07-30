use sqlx::Row;
use rust_decimal::prelude::ToPrimitive;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Expense {
    id: i32,
    user_id: i32,
    category_id: i32,

    amount: f64,
    description: String,

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
        purchased_at: chrono::NaiveDate,
        user_owes: Vec<UserOwes>,
    ) -> Self {
        Self {
            id: -1,
            user_id,
            category_id,
            amount,
            description,
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
