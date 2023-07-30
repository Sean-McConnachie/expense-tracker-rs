use crate::database;

pub mod args;
pub mod config;
pub mod logger;

pub async fn startup(config: &config::Config, db_pool: &sqlx::PgPool) -> anyhow::Result<()> {
    database::initialize::initialize_db(&db_pool)
        .await
        .expect("Failed to initialize database.");

    // Insert default categories
    //let categories = config.categories();
    //for category in categories {
        //database::category::insert_category(&db_pool, category.name.clone(), &category.description.clone())
            //.await
            //.expect("Failed to insert default category.");
    //}

    Ok(())
}
