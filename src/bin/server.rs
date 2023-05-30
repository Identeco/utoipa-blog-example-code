use actix_web::{App, HttpServer};
use identeco_utoipa_example::{check_credentials, ApiDocs};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() {
    let openapi = ApiDocs::openapi();
    HttpServer::new(move || {
        App::new().service(check_credentials).service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
        )
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
    .unwrap();
}
