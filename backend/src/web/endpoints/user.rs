use rocket::serde::json::Json;
use rocket::{delete, get, post, State};

use crate::database::user;
use datatypes::User;

#[post("/create", format = "json", data = "<name>")]
pub async fn users_create(
    db_pool: &State<sqlx::PgPool>,
    name: Json<String>,
) -> Result<Json<User>, std::io::Error> {
    let user = user::insert_user(db_pool, name.0)
        .await
        .map_err(|_e| std::io::Error::new(std::io::ErrorKind::Other, "Failed to create user"))?;

    Ok(Json(user))
}

#[get("/all")]
pub async fn users_all(
    db_pool: &State<sqlx::PgPool>,
) -> Result<Json<Vec<User>>, std::io::Error> {
    let users = user::get_users(db_pool)
        .await
        .map_err(|_e| std::io::Error::new(std::io::ErrorKind::Other, "Failed to get users"))?;

    Ok(Json(users))
}

#[delete("/delete", format = "json", data = "<user_id>")]
pub async fn users_delete(
    db_pool: &State<sqlx::PgPool>,
    user_id: Json<i32>,
) -> Result<Json<bool>, std::io::Error> {
    let user = user::delete_user(db_pool, user_id.0)
        .await
        .map_err(|_e| std::io::Error::new(std::io::ErrorKind::Other, "Failed to delete user"))?;

    Ok(Json(user))
}
