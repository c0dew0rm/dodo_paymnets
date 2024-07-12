use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;
use bcrypt::{hash, verify};
use crate::models::{User, NewUser, LoginUser};
use crate::schema::users::dsl::*;
use crate::utils::{create_jwt, decode_jwt};
use crate::DbPool;
use serde_json::json;
use log::{info, error};



#[derive(Clone)]
pub struct AppState {
    pub pool: web::Data<DbPool>,
    pub secret_key: String,
}

pub async fn register_user(state: web::Data<AppState>, new_user: web::Json<NewUser>) -> HttpResponse {
    let mut conn = state.pool.get().expect("Couldn't get db connection from pool");

    let hashed_password = hash(&new_user.password, 4).unwrap();
    let new_user = NewUser {
        username: new_user.username.clone(),
        password: hashed_password,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn)
        .expect("Error registering user");

        HttpResponse::Ok().json(json!({
            "msg": "SUCCESS",
        }))
}

pub async fn login_user(state: web::Data<AppState>, login: web::Json<LoginUser>) -> HttpResponse {
    let mut conn = state.pool.get().expect("Couldn't get db connection from pool");

    let user = users
        .filter(username.eq(&login.username))
        .first::<User>(&mut conn)
        .expect("User not found");

    if verify(&login.password, &user.password).unwrap() {
        let token = create_jwt(&user.id.to_string(), &state.secret_key);
        HttpResponse::Ok().json(json!({
            "msg": "SUCCESS",
            "token": token
        }))
    } else {
        HttpResponse::Unauthorized().json(json!({
            "msg": "UNAUTHORIZED",
        }))
    }
}

pub async fn get_user_profile(state: web::Data<AppState>, token: web::Json<String>) -> HttpResponse {
    let mut conn = state.pool.get().expect("Couldn't get db connection from pool");
    let claims = decode_jwt(&token, &state.secret_key);
    let user_id = Uuid::parse_str(&claims.claims.sub).unwrap();

    let user = users.find(user_id).first::<User>(&mut conn).expect("User not found");
    HttpResponse::Ok().json(json!({
        "msg": "SUCCESS",
        "data": user
    }))
    HttpResponse::Ok().finish()
}
