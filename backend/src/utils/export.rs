use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use rocket::serde::json::serde_json;
use crate::database;
use crate::utils::JsonFormat;


pub async fn export_json(db_pool: &sqlx::PgPool, path: &str) -> anyhow::Result<()> {
    let mut input = String::new();
    let fp = PathBuf::from(path);
    if fp.exists() {
        println!("File already exists. Overwrite? [y/n]");
        io::stdin().read_line(&mut input)?;
        if input.trim() != "y" {
            return Ok(());
        }
    }
    input.clear();

    let expenses = database::expense::get_expenses(&db_pool, None).await?;
    let users = database::user::get_users(&db_pool).await?;
    let categories = database::category::get_categories(&db_pool).await?;

    let json_format = JsonFormat::new(users, categories, expenses);

    let json = serde_json::to_string(&json_format)?;
    let mut file = File::create(fp)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}