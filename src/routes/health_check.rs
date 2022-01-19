use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod health_check {
    use crate::routes::health_check;

    #[tokio::test]
    async fn should_return_success() {
        let response = health_check().await;

        assert!(response.status().is_success());
    }
}
