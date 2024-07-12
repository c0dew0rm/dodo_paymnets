#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use crate::handlers::transaction::{create_transaction, get_transactions};

    #[actix_web::test]
    async fn test_create_transaction() {
        let mut app = test::init_service(App::new().route("/transactions", web::post().to(create_transaction))).await;
        let payload = r#"{"user_id": "user_id", "amount": 100.0, "description": "Test transaction"}"#.to_string();

        let req = test::TestRequest::post().uri("/transactions").set_payload(payload).to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_transactions() {
        let mut app = test::init_service(App::new().route("/transactions", web::get().to(get_transactions))).await;
        let token = r#"your_jwt_token"#.to_string();

        let req = test::TestRequest::get().uri("/transactions").set_payload(token).to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }
}
