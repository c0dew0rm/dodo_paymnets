// @generated automatically by Diesel CLI.

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Varchar,
        balance -> Double,
    }
}

table! {
    transactions (id) {
        id -> Uuid,
        user_id -> Uuid,
        amount -> Double,
        description -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(transactions -> users (user_id));
allow_tables_to_appear_in_same_query!(users, transactions);
