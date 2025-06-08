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

fn install_tables(pool: &PgPool) -> Result<(), sqlx::Error> {
    let create_users_table = r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(50) UNIQUE NOT NULL,
            password VARCHAR(255) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
    "#;

    sqlx::query(create_users_table).execute(pool).await?;
    Ok(())
}

fn main() {
    // Carregando o arquivo .env
    from_path(StdPath::new("../.env")).expect("Falha ao carregar .env");
    
    // Iniciando conexão com o banco de dados
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool");

    // Instalação das tabelas no banco de dados
    install_tables(&pool).expect("Failed to install database tables");
    println!("finalizado");
}
