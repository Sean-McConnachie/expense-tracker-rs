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

pub async fn insert_user(db_pool: &sqlx::PgPool, username: String) -> Result<User, sqlx::Error> {
    let sql = r#"
    INSERT INTO users (username)
    VALUES ($1)
    RETURNING id, username, created_at
    "#;
    let row = sqlx::query(sql).bind(username).fetch_one(db_pool).await?;
    Ok(User::from(row))
}

pub async fn get_users(db_pool: &sqlx::PgPool) -> Result<Vec<User>, sqlx::Error> {
    let sql = r#"
    SELECT id, username, created_at
    FROM users
    "#;
    let rows = sqlx::query(sql).fetch_all(db_pool).await?;
    let mut users = Vec::new();
    for row in rows {
        users.push(User::from(row));
    }
    Ok(users)
}

pub async fn delete_user(db_pool: &sqlx::PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let sql = r#"
    DELETE FROM users
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
    fn test_user_new() {
        let _user = User::new("test".to_string());
    }

    #[tokio::test]
    async fn test_insert() {
        let db_config = crate::database::DbConfig {
            database_url: "postgres://postgres:12341234@localhost/expense_tracker".to_string(),
            max_connections: 5,
        };
        let db_pool = crate::database::connect_db(&db_config).await.unwrap();

        let user = insert_user(&db_pool, "test1".to_string()).await.unwrap();

        assert_eq!(user.username(), "test1");
    }

    #[tokio::test]
    async fn test_get_users() {
        let db_config = crate::database::DbConfig {
            database_url: "postgres://postgres:12341234@localhost/expense_tracker".to_string(),
            max_connections: 5,
        };
        let db_pool = crate::database::connect_db(&db_config).await.unwrap();

        let users = get_users(&db_pool).await.unwrap();
        dbg!(&users);
    }

    #[tokio::test]
    async fn test_delete_user_by_id() {
        let db_config = crate::database::DbConfig {
            database_url: "postgres://postgres:12341234@localhost/expense_tracker".to_string(),
            max_connections: 5,
        };
        let db_pool = crate::database::connect_db(&db_config).await.unwrap();

        let users = get_users(&db_pool).await.unwrap();

        let user = users.first().unwrap().clone();

        assert_eq!(delete_user(&db_pool, user.id()).await.unwrap(), true);
        assert_eq!(delete_user(&db_pool, user.id()).await.unwrap(), false);
    }
}
