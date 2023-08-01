pub async fn initialize_db(db_pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let sql = r#"
    CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        username VARCHAR(255) NOT NULL UNIQUE,
        created_at TIMESTAMP NOT NULL DEFAULT NOW()
    );
    "#;
    sqlx::query(sql).execute(db_pool).await?;

    let sql = r#"
    CREATE TABLE IF NOT EXISTS categories (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL UNIQUE,
        description VARCHAR(255) NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT NOW()
    );
    "#;
    sqlx::query(sql).execute(db_pool).await?;

    let sql = r#"
    CREATE TABLE IF NOT EXISTS expenses (
        id SERIAL PRIMARY KEY,
        user_id INTEGER NOT NULL REFERENCES users(id),
        category_id INTEGER NOT NULL REFERENCES categories(id),

        amount NUMERIC(10, 3) NOT NULL,
        description VARCHAR(255) NOT NULL,

        created_at TIMESTAMP NOT NULL DEFAULT NOW(),
        purchased_at DATE NOT NULL
    );
    "#;
    sqlx::query(sql).execute(db_pool).await?;

    let sql = r#"
    CREATE TABLE IF NOT EXISTS user_owes (
        id SERIAL PRIMARY KEY,
        user_id INTEGER NOT NULL REFERENCES users(id),
        expense_id INTEGER NOT NULL REFERENCES expenses(id),
        amount NUMERIC(10, 3) NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT NOW()
    );
    "#;
    sqlx::query(sql).execute(db_pool).await?;

    let sql = r#"
    CREATE TABLE IF NOT EXISTS cleared_from (
        id SERIAL PRIMARY KEY,
        date TIMESTAMP NOT NULL DEFAULT NOW()
    );
    "#;
    sqlx::query(sql).execute(db_pool).await?;

    Ok(())
}
