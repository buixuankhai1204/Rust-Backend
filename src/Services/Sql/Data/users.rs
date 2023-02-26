use serde::{Serialize, Deserialize};
use std::io::{Read, Write};
use std::pin::Pin;
use tokio_postgres::{Error, Row, RowStream};
use crate::Models::RequestUser;

#[derive(Deserialize,Serialize,Debug, Clone)]
pub struct user_register{
    pub id: i32,
    pub username: String,
    pub fullname: String,
    pub email: String,
    pub address: String,
}

impl From<Row> for user_register {
    fn from(row: Row) -> Self {
        let id : i32 = row.get("id");
        Self {
            id: id,
            username: row.get("username"),
            fullname: row.get("fullname"),
            email: row.get("email"),
            address: row.get("address"),
        }
    }
}
