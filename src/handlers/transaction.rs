use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;
use crate::models::{Transaction, NewTransaction};
use crate::schema::transactions::dsl::*;
use crate::utils::decode_jwt;
use crate::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: web::Data<DbPool>,
    pub secret_key: String,
}

pub async fn create_transaction(state: web::Data<AppState>, token: web::Json<String>, new_transaction: web::Json<NewTransaction>) -> HttpResponse {
    let mut conn = state.pool.get().expect("Couldn't get db connection from pool");

    let claims = decode_jwt(&token, &state.secret_key);
    let other_user_id = Uuid::parse_str(&claims.claims.sub).unwrap();

    let new_transaction = NewTransaction {
        user_id: other_user_id,
        amount: new_transaction.amount,
        description: new_transaction.description.clone(),
    };

    diesel::insert_into(transactions)
        .values(&new_transaction)
        .execute(&mut conn)
        .expect("Error creating transaction");

    HttpResponse::Ok().finish()
}

pub async fn get_transactions(state: web::Data<AppState>, token: web::Json<String>) -> HttpResponse {
    let mut conn = state.pool.get().expect("Couldn't get db connection from pool");

    let claims = decode_jwt(&token, &state.secret_key);
    let other_user_id = Uuid::parse_str(&claims.claims.sub).unwrap();

    let results = transactions.filter(user_id.eq(other_user_id))
        .load::<Transaction>(&mut conn)
        .expect("Error loading transactions");

    HttpResponse::Ok().json(results)
}
