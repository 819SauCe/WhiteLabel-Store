mod functions;
use axum::{
    extract::{Path, State, Multipart, TypedHeader},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post, put, delete},
    Json, Router, Server,
};
use headers::Authorization;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use std::{env,fs::{File},io::Write,net::SocketAddr,path::{Path as StdPath, PathBuf},};
use dotenvy::from_path;
use tower_http::cors::{Any, CorsLayer};
use jsonwebtoken::{encode, EncodingKey, DecodingKey, Header, Validation};
use chrono::{Utc, Duration};
use headers::authorization::Bearer;
use tower_http::services::ServeDir;
use chrono::NaiveDateTime;
use once_cell::sync::Lazy;
use bcrypt::{DEFAULT_COST, hash, verify};
use crate::functions::install::create_all_tables;
use crate::functions::route::*;

#[tokio::main]
async fn main() {
    dotenvy::from_path(std::path::Path::new("../.env")).expect("Falha ao carregar .env");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to the database");

    create_all_tables(&pool).await;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/produtos/register", post(product_register).with_state(pool.clone()))
        .route("/produtos", get(product_list).with_state(pool.clone()))
        .route("/produtos/:id", get(product_details).with_state(pool.clone()))
        .layer(cors);
   
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
