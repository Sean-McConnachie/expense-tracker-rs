use rocket::serde::json::Json;
use rocket::{delete, get, post, State};

use crate::database::category;

#[post("/create", format = "json", data = "<name_description>")]
pub async fn categories_create(
    db_pool: &State<sqlx::PgPool>,
    name_description: Json<(String, String)>,
) -> Result<Json<category::Category>, std::io::Error> {
    let category = category::insert_category(db_pool, name_description.0 .0, name_description.0 .1)
        .await
        .map_err(|_e| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to create category")
        })?;

    Ok(Json(category))
}

#[get("/all")]
pub async fn categories_all(
    db_pool: &State<sqlx::PgPool>,
) -> Result<Json<Vec<category::Category>>, std::io::Error> {
    let categories = category::get_categories(db_pool)
        .await
        .map_err(|_e| std::io::Error::new(std::io::ErrorKind::Other, "Failed to get categories"))?;

    Ok(Json(categories))
}

#[delete("/delete", format = "json", data = "<category_id>")]
pub async fn categories_delete(
    db_pool: &State<sqlx::PgPool>,
    category_id: Json<i32>,
) -> Result<Json<bool>, std::io::Error> {
    let category = category::delete_category(db_pool, category_id.0)
        .await
        .map_err(|_e| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to delete category")
        })?;

    Ok(Json(category))
}
