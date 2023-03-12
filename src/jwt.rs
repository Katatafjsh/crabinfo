use serde::{Deserialize, Serialize};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use chrono::{Utc, Duration};

use axum_auth::AuthBearer;

use axum::{
    Json,
    Router, 
    http::{StatusCode},
    routing::{get, post}, 
    response::{IntoResponse, Response},
};

pub fn routes() -> Router {
    Router::new()
        .route("/token", post(jwt_generate))
        .route("/token/validate", get(jwt_validate))
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
}

impl Claims {
    pub fn new(sub: String, ttl: i64) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::seconds(ttl);

        Self {
            sub: sub,
            iat: iat.timestamp() as usize,
            exp: exp.timestamp() as usize,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    token: String,
    expires_at: usize
}
#[derive(Debug, Serialize, Deserialize)]
struct TokenValidationResponse {
	token_name: String,
    expires_at: usize
}

async fn jwt_generate() -> Response {
    // TODO: pass subject in body
    // TODO: TokenResponse: ISO 8601
    // TODO: feat: encode jwt with cert

    let jwt_secret = "secret".to_owned();
    let sub = "anonymous".to_owned();
    let ttl = 60;

    let claims = &Claims::new(sub, ttl);

    let token = jsonwebtoken::encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()));

    match token{
        Ok(t) => Json(TokenResponse { token: t, expires_at: claims.exp }).into_response(),
        Err(_) => (StatusCode::BAD_REQUEST).into_response(),
    }
}

async fn jwt_validate(AuthBearer(token): AuthBearer) -> Response {
    // TODO: is axum_auth needed?

    let jwt_secret = "secret".to_owned();

    match jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(c) => Json(TokenValidationResponse { token_name: c.claims.sub, expires_at: c.claims.exp }).into_response(),
        Err(_) => StatusCode::UNAUTHORIZED.into_response(),
    }
}
