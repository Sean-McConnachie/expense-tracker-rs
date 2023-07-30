use datatypes::Category;

pub async fn insert_category(
    db_pool: &sqlx::PgPool,
    name: String,
    description: &String,
) -> Result<Category, sqlx::Error> {
    let sql = r#"
    INSERT INTO categories (name, description)
    VALUES ($1, $2)
    RETURNING id, name, description,  created_at
    "#;
    let row = sqlx::query(sql)
        .bind(name)
        .bind(description)
        .fetch_one(db_pool)
        .await?;
    Ok(Category::from(row))
}

pub async fn get_categories(db_pool: &sqlx::PgPool) -> Result<Vec<Category>, sqlx::Error> {
    let sql = r#"
    SELECT id, name, description, created_at
    FROM categories
    "#;
    let rows = sqlx::query(sql).fetch_all(db_pool).await?;
    let mut users = Vec::new();
    for row in rows {
        users.push(Category::from(row));
    }
    Ok(users)
}

pub async fn delete_category(db_pool: &sqlx::PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let sql = r#"
    DELETE FROM categories
    WHERE id = $1
    RETURNING id
    "#;
    match sqlx::query(sql).bind(id).fetch_one(db_pool).await {
        Ok(_) => Ok(true),
        Err(e) => match e {
            sqlx::Error::RowNotFound => Ok(false),
            _ => Err(e),
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_category_new() {
        let _user = Category::new("test".to_string(), "description".to_string());
    }

    #[tokio::test]
    async fn test_insert() {
        let db_config = crate::database::DbConfig {
            database_url: "postgres://postgres:12341234@localhost/expense_tracker".to_string(),
            max_connections: 5,
        };
        let db_pool = crate::database::connect_db(&db_config).await.unwrap();

        let user = insert_category(&db_pool, "test".to_string(), "description".to_string())
            .await
            .unwrap();

        assert_eq!(user.name(), "test");
        assert_eq!(user.description(), "description");
    }

    #[tokio::test]
    async fn test_get_categories() {
        let db_config = crate::database::DbConfig {
            database_url: "postgres://postgres:12341234@localhost/expense_tracker".to_string(),
            max_connections: 5,
        };
        let db_pool = crate::database::connect_db(&db_config).await.unwrap();

        let categories = get_categories(&db_pool).await.unwrap();
        dbg!(&categories);
    }

    #[tokio::test]
    async fn test_delete_category_by_id() {
        let db_config = crate::database::DbConfig {
            database_url: "postgres://postgres:12341234@localhost/expense_tracker".to_string(),
            max_connections: 5,
        };
        let db_pool = crate::database::connect_db(&db_config).await.unwrap();

        let categories = get_categories(&db_pool).await.unwrap();

        let category = categories.first().unwrap().clone();

        assert_eq!(
            delete_category(&db_pool, category.id()).await.unwrap(),
            true
        );
        assert_eq!(
            delete_category(&db_pool, category.id()).await.unwrap(),
            false
        );
    }
}
