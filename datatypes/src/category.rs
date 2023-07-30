use serde::{Serialize, Deserialize};
use sqlx::Row;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Category {
    id: i32,
    name: String,
    description: String,
    created_at: chrono::NaiveDateTime,
}



impl Category {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: -1,
            name,
            description,
            created_at: NaiveDateTime::from_timestamp_millis(0).unwrap(),
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn created_at(&self) -> &chrono::NaiveDateTime {
        &self.created_at
    }
}

impl From<sqlx::postgres::PgRow> for Category {
    fn from(row: sqlx::postgres::PgRow) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            created_at: row.get("created_at"),
        }
    }
}
