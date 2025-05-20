use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::prelude::FromRow;
use sqlx::Row;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:555@localhost:5432/postgres")
        .await?;
    //sqlx::migrate!("./migrations").run(&pool).await?;
    let user: User = User {
        id: "2".to_string(),
        name: "ahmad".to_string(),
    };
    //create_user(&user, &pool).await?;
    let all = get_all_users(&pool).await?;

    println!("Hello, world! {:#?}", all);
    Ok(())
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    id: String,
    name: String,
}

pub async fn create_user(user: &User, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let q = sqlx::query("insert into users(id,name) values ($1,$2)")
        .bind(user.id.clone())
        .bind(user.name.clone())
        .execute(pool)
        .await?;
    Ok(())
}

// pub async fn get_users(pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
//     let q = sqlx::query("select * from users")
//         .bind(user.id)
//         .bind(user.name.clone())
//         .execute(pool)
//         .await?;
//     Ok(())
// }

use sqlx::query_as;

async fn get_all_users(pool: &sqlx::PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = query_as!(User, "SELECT id, name FROM users")
        .fetch_all(pool)
        .await?;

    Ok(users)
}

// $env:DATABASE_URL="postgres://postgres:555@localhost:5432/postgres"
