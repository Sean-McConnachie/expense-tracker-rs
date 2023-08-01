use std::fmt::Display;
use sqlx::Row;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct User {
    id: i32,
    username: String,
    created_at: chrono::NaiveDateTime,
}

impl User {
    pub fn new(username: String) -> Self {
        Self {
            id: -1,
            username,
            created_at: chrono::NaiveDateTime::from_timestamp_millis(0).unwrap(),
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn created_at(&self) -> &chrono::NaiveDateTime {
        &self.created_at
    }
}

impl From<sqlx::postgres::PgRow> for User {
    fn from(row: sqlx::postgres::PgRow) -> Self {
        Self {
            id: row.get("id"),
            username: row.get("username"),
            created_at: row.get("created_at"),
        }
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let created_at = self.created_at().format("%Y-%m-%d %H:%M:%S");
        write!(f, "{}: {} ({})", self.id(), self.username(), created_at)
    }
}