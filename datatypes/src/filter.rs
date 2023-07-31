#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum OrderBy {
    Amount,
    Date,
    Created,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Filter {
    pub user_ids: Vec<i32>,

    pub category_ids: Vec<i32>,

    pub min_amount: f64,
    pub max_amount: f64,

    pub min_date: chrono::NaiveDate,
    pub max_date: chrono::NaiveDate,

    pub order_by: OrderBy,
    pub order_asc: bool,
}

impl Filter {
    pub fn user_ids(&self) -> &Vec<i32> {
        &self.user_ids
    }

    pub fn category_ids(&self) -> &Vec<i32> {
        &self.category_ids
    }

    pub fn min_amount(&self) -> f64 {
        self.min_amount
    }

    pub fn max_amount(&self) -> f64 {
        self.max_amount
    }

    pub fn min_date(&self) -> &chrono::NaiveDate {
        &self.min_date
    }

    pub fn max_date(&self) -> &chrono::NaiveDate {
        &self.max_date
    }

    pub fn order_by(&self) -> &OrderBy {
        &self.order_by
    }

    pub fn order_asc(&self) -> bool {
        self.order_asc
    }
}
