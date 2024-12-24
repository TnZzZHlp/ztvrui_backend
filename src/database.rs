use std::fs::File;
use sqlx::{ query, Pool, Sqlite, Row };
use bcrypt::{ DEFAULT_COST, hash, verify };

pub struct Database {
    pub conn: Pool<Sqlite>,
}

impl Database {
    pub fn new(config: &crate::config::AppConfig) -> Self {
        // Create database file if it doesn't exist
        if !std::path::Path::new(&config.database_path).exists() {
            File::create(&config.database_path).unwrap();
        }

        let conn = Pool::connect_lazy(&config.database_path);

        match conn {
            Ok(conn) => Self { conn },
            Err(e) => {
                eprintln!("Failed to connect to database: {}", e);
                std::process::exit(1);
            }
        }
    }

    pub async fn init(&self) {
        // USERS table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS USERS (
                ID INTEGER PRIMARY KEY AUTOINCREMENT,
                USERNAME TEXT NOT NULL,
                PASSWORD TEXT NOT NULL,
                COOKIE TEXT,
                CREATED_AT TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        "#
        )
            .execute(&self.conn).await
            .expect("Failed to create table");

        // Default user
        // admin
        // password
        sqlx::query(
            r#"
        INSERT INTO USERS (USERNAME, PASSWORD)
        SELECT ?, ?
        WHERE NOT EXISTS (
            SELECT 1 FROM USERS WHERE USERNAME = ?
        )
        "#
        )
            .bind("admin")
            .bind("$2b$08$ygLvX.7v3/IRSxgsp/dBF.nNE1uTb4Nd0IWQHurbC6bnzr8Hru.Ly")
            .bind("admin")
            .execute(&self.conn).await
            .expect("Failed to insert default user");
    }

    pub async fn verify(&self, username: &str, password: &str) -> bool {
        let row = query(r#"SELECT PASSWORD FROM USERS WHERE USERNAME = $1"#)
            .bind(username)
            .fetch_one(&self.conn).await;

        if let Ok(row) = row {
            verify(password, &row.get::<String, _>("PASSWORD")).unwrap()
        } else {
            false
        }
    }

    pub async fn update_user_cookie(
        &self,
        username: &str,
        cookie: &str
    ) -> Result<(), sqlx::Error> {
        query(r#"UPDATE USERS SET COOKIE = $1 WHERE USERNAME = $2"#)
            .bind(cookie)
            .bind(username)
            .execute(&self.conn).await?;

        Ok(())
    }

    pub async fn verify_cookie(&self, cookie: &str) -> bool {
        let row = query(r#"SELECT COOKIE FROM USERS WHERE COOKIE = $1"#)
            .bind(cookie)
            .fetch_all(&self.conn).await;

        if let Ok(row) = row {
            !row.is_empty()
        } else {
            false
        }
    }

    pub async fn remove_cookie(&self, cookie: &str) -> Result<(), sqlx::Error> {
        query(r#"UPDATE USERS SET COOKIE = NULL WHERE COOKIE = $1"#)
            .bind(cookie)
            .execute(&self.conn).await?;

        Ok(())
    }

    pub async fn update_user_info(
        &self,
        username: &str,
        password: &str
    ) -> Result<(), sqlx::Error> {
        query(r#"UPDATE USERS SET PASSWORD = $1 WHERE USERNAME = $2"#)
            .bind(hash(password, DEFAULT_COST).unwrap())
            .bind(username)
            .execute(&self.conn).await?;

        Ok(())
    }
}
