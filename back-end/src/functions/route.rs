use axum::{extract::{Path, State},http::StatusCode,Json,};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(FromRow, serde::Serialize)]
pub struct Product {
    pub id: i32,
    pub nome: String,
    pub marca: String,
    pub cor: String,
    pub valor: f64,
    pub desconto: f64,
    pub descricao: String,
    pub categoria_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct ProductRegister {
    pub nome: String,
    pub marca: String,
    pub cor: String,
    pub valor: f64,
    pub desconto: f64,
    pub descricao: String,
    pub categoria_id: i32,
    pub loja_id: i32,
    pub registrado_por: i32,
}

#[derive(Debug, Deserialize, sqlx::FromRow)]
pub struct ProductOverview {
    pub id: i32,
    pub nome: String,
    pub marca: String,
    pub cor: String,
    pub valor: f64,
    pub desconto: f64,
    pub descricao: String,
    pub categoria_id: i32,
}


pub async fn product_register(State(pool): State<PgPool>,Json(produto): Json<ProductRegister>,) -> Result<String, (axum::http::StatusCode, String)> {
    println!("{:?}", produto);
    let query = r#"INSERT INTO produtos (nome, marca, cor, valor, desconto, descricao, categoria_id, loja_id, registrado_por)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
    "#;

    sqlx::query(query)
        .bind(&produto.nome)
        .bind(&produto.marca)
        .bind(&produto.cor)
        .bind(produto.valor)
        .bind(produto.desconto)
        .bind(&produto.descricao)
        .bind(produto.categoria_id)
        .bind(produto.loja_id)
        .bind(produto.registrado_por)
        .execute(&pool)
        .await
        .map_err(|e| {
        println!("Erro ao inserir produto: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;


    Ok("Produto registrado com sucesso".into())
}

pub async fn product_list(State(pool): State<PgPool>) -> Result<Json<Vec<Product>>, (StatusCode, String)> {
    let query = r#"SELECT id, nome, marca, cor, valor, desconto, descricao, categoria_id FROM produtos"#;

    let produtos: Vec<Product> = sqlx::query_as(query)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            println!("Erro ao buscar produtos: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok(Json(produtos))
}

pub async fn product_details(Path(id): Path<i32>, State(pool): State<PgPool>) -> Result<Json<Product>, (StatusCode, String)> {
    let query = "SELECT * FROM produtos WHERE id = $1";
    let produto = sqlx::query_as::<_, Product>(query)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            println!("Erro ao buscar produto por id: {:?}", e);
            (StatusCode::NOT_FOUND, e.to_string())
        })?;

    Ok(Json(produto))
}