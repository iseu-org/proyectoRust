use axum::{
    extract::Query,
    http::{StatusCode, Method, HeaderValue},
    response::Json,
    routing::get,
    Router,
};
use axum_extra::{headers::UserAgent, TypedHeader};
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tower_http::cors::{CorsLayer, Any};

#[derive(Deserialize)]
struct InfoQuery {
    name: String,
}

#[derive(Deserialize)]
struct TokenQuery {
    token: String,
}

#[derive(Serialize)]
struct InfoResponse {
    greeting: String,
    user_agent: String,
    timestamp: String,
}

#[derive(Serialize, Deserialize)]
struct GitHubUser {
    login: String,
    name: Option<String>,
    email: Option<String>,
    public_repos: u32,
    followers: u32,
    following: u32,
    disk_usage: Option<u64>,
    created_at: String,
    bio: Option<String>,
    avatar_url: String,
}

async fn info_handler(
    Query(params): Query<InfoQuery>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> Result<Json<InfoResponse>, (StatusCode, String)> {
    if params.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "El parámetro 'name' no puede estar vacío".to_string(),
        ));
    }

    Ok(Json(InfoResponse {
        greeting: format!("Hola, {}!", params.name),
        user_agent: user_agent.to_string(),
        timestamp: Utc::now().to_rfc3339(),
    }))
}

async fn github_handler(
    Query(params): Query<TokenQuery>,
) -> Result<Json<GitHubUser>, (StatusCode, String)> {
    if params.token.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "El parámetro 'token' no puede estar vacío".to_string(),
        ));
    }

    let client = Client::new();

    let response = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", params.token))
        .header("User-Agent", "apiRust/1.0")
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !response.status().is_success() {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Token inválido o expirado".to_string(),
        ));
    }

    let user = response
        .json::<GitHubUser>()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(user))
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET])
        .allow_headers(Any);

    let app = Router::new()
        .route("/info", get(info_handler))
        .route("/github", get(github_handler))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Servidor corriendo en http://localhost:3000");
    println!("📌 GET /info?name=<nombre>");
    println!("📌 GET /github?token=<tu_token_github>");
    axum::serve(listener, app).await.unwrap();
}