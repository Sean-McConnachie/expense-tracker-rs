mod cors;
mod endpoints;
mod error;

use endpoints::*;

use crate::utils::config;
use cors::*;
use rocket::routes;

pub async fn run(config: config::Config, db_pool: sqlx::PgPool) {
    rocket::build()
        .mount(
            "/categories",
            routes![categories_create, categories_all, categories_delete],
        )
        .mount(
            "/expenses",
            routes![expenses_create, expenses_all, expenses_filter, expenses_last_reset]
        )
        .mount("/users", routes![users_create, users_all, users_delete])
        .attach(CORS)
        .manage(db_pool)
        .manage(config)
        .launch()
        .await
        .expect("Bye bye server...");
}
