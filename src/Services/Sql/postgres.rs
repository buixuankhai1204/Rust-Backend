use std::io::{Read, Write};
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tokio_postgres::error::SqlState;
use tokio_postgres::types::Type;
use tokio_postgres::{NoTls, Row, Error};
use crate::Models::RequestUser;
use crate::Services::Sql::Data;
use crate::Services::Sql::Data::users::user_register;

 pub async fn insert_user(user: & RequestUser) -> Result<Vec<user_register>, Error>
{
    let (client, connection) = tokio_postgres::connect(
        "jdbc:postgresql://localhost:5432/postgres",
        NoTls,
    ).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    
    let result = client.query("SELECT * FROM users", &[]).await.unwrap();

    let users: Vec<user_register> = result.into_iter().map(|row| user_register::from(row)).collect();
    Ok(users)
}

pub async fn get_user() -> Result<Vec<user_register>, Error>
{
    let (client, connection) = tokio_postgres::connect(
        "host=localhost port=5432 user=postgres password=postgrespw",
        NoTls,
    ).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    
    let result = client.query("SELECT * FROM users", &[]).await?;

    let users: Vec<user_register> = result.into_iter().map(|row| user_register::from(row)).collect();
    Ok(users)
}