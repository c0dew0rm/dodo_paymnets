use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::schema::{users, transactions};

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub balance: f64,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Token {
    pub token: String,
}


#[derive(Queryable, Serialize)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct AuthPayload {
    pub sub: String,
    pub exp: usize,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "transactions"]
pub struct NewTransaction {
    pub user_id: Uuid,
    pub amount: f64,
    pub description: String,
}
