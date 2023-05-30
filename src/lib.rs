use actix_web::{http::StatusCode, post, web::Json, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi, ToSchema,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Credential Check API definition",
        description = "Enables clients to anonymously check whether their credentials have been involved in a known data breach.",
        contact(
            name = "Identeco",
            email = "contact@identeco.de",
            url = "https://identeco.de"
        ),
        version = "1.0.2"
    ),
    servers((url = "http://localhost:8080/", description  = "Local development server")),
    components(schemas(CcRequest, CcResponse)),
    paths(check_credentials),
    security(("My-Api-Key" = [])),
    modifiers(&SecuritySchemas),
)]
pub struct ApiDocs;

impl ApiDocs {
    pub fn generate() -> String {
        ApiDocs::openapi()
            .to_yaml()
            .expect("Could not generate OpenAPI spec")
    }
}

/// Custom security schemas for the API.
struct SecuritySchemas;

impl Modify for SecuritySchemas {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        let value = ApiKeyValue::with_description("My-Api-Key", "Custom authentication header");
        let scheme = SecurityScheme::ApiKey(ApiKey::Header(value));
        components.add_security_scheme("My-Api-Key", scheme);
    }
}

/// The request body for the `check_credentials` endpoint.
#[derive(ToSchema, Debug, Serialize, Deserialize)]
struct CcRequest {
    /// Prefix of the hashed email
    #[schema(format = "base64", example = "cri4", min_length = 4, max_length = 6)]
    pub prefix: String,

    /// Cryptographically treated email-password combination
    #[schema(format = "base64", example = "02a8902230d79486d10ec6eb6")]
    pub credentials: String,
}

/// The response body for the `check_credentials` endpoint.
#[derive(ToSchema, Debug, Serialize, Deserialize)]
struct CcResponse {
    /// The double blinded user input
    #[schema(format = "base64", example = "0326da340f354cec41be1c1246")]
    pub credentials: String,

    /// Cryptographically treated email-password combinations associated with the
    /// user provided prefix
    #[schema(format = "base64", example = "de6ceb426447319caff8d28f42")]
    pub matches: Vec<String>,
}

/// Errors that can occur when checking credentials.
#[derive(Debug, thiserror::Error)]
enum ApiError {
    #[error("Prefix has a length of {len}, but the minimum is 4")]
    PrefixTooShort { len: usize },
    #[error("Prefix has a length of {len}, but the maximum is 6")]
    PrefixTooLong { len: usize },
}

/// Used in actix web to convert `ApiError` to an HTTP response.
impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::PrefixTooShort { .. } => StatusCode::BAD_REQUEST,
            ApiError::PrefixTooLong { .. } => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type("text/plain")
            .body(self.to_string())
    }
}

/// Checks whether an email-password combination is known to been involved in a data breach.
#[utoipa::path(
    request_body = CcRequest,
    responses((
        status = OK,
        body = CcResponse,
    ),(
        status = BAD_REQUEST,
        description = "The request body contained ill formatted values",
        body = String,
        examples(
            ("TooShort" = (value = 
                json!(ApiError::PrefixTooShort { len: 3 }.to_string()))
            ),
            ("TooLong" = (value = 
                json!(ApiError::PrefixTooLong { len: 7 }.to_string()))
            )
        ),
    )),
)]
#[post("/check_credentials")]
async fn check_credentials(body: Json<CcRequest>) -> Result<Json<CcResponse>, ApiError> {
    let len = body.prefix.len();
    if len > 6 {
        return Err(ApiError::PrefixTooLong { len });
    }
    if len < 4 {
        return Err(ApiError::PrefixTooShort { len });
    }

    Ok(Json(CcResponse {
        credentials: "placeholder".to_string(),
        matches: vec!["placeholder".to_string()],
    }))
}

#[test]
fn generated_docs_are_up_to_date() {
    let path = "openapi.yml";
    let current = std::fs::read_to_string(path).expect("The current openapi file must exist");
    let newest = ApiDocs::generate();

    assert_eq!(
        newest, current,
        "
============================================================
NOTE: The generated `{path}` file is not up to date.
Please run `cargo run --bin gen_api` and commit the changes.
============================================================
"
    )
}
