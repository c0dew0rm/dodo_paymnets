#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use crate::handlers::user::{register_user, login_user};

    #[actix_web::test]
    async fn test_register_user() {
        let mut app = test::init_service(App::new().route("/register", web::post().to(register_user))).await;
        let payload = r#"{"username": "testuser", "password": "password"}"#.to_string();

        let req = test::TestRequest::post().uri("/register").set_payload(payload).to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_login_user() {
        let mut app = test::init_service(App::new().route("/login", web::post().to(login_user))).await;
        let payload = r#"{"username": "testuser", "password": "password"}"#.to_string();

        let req = test::TestRequest::post().uri("/login").set_payload(payload).to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }
}
