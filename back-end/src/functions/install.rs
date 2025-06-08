/*
DROP TABLE IF EXISTS
    imagens,
    produto_cupons,
    produtos,
    cupons,
    categorias,
    usuarios,
    users,
    lojas
CASCADE;
*/


use sqlx::PgPool;

pub async fn create_lojas(pool: &PgPool) -> Result<(), sqlx::Error> {
    let query = r#"
        CREATE TABLE IF NOT EXISTS lojas (
            id SERIAL PRIMARY KEY,
            nome VARCHAR(100) NOT NULL
        );
    "#;

    sqlx::query(query).execute(pool).await?;
    Ok(())
}

pub async fn create_usuarios(pool: &PgPool) -> Result<(), sqlx::Error> {
    let query = r#"
        CREATE TABLE IF NOT EXISTS usuarios (
            id SERIAL PRIMARY KEY,
            nome VARCHAR(100) NOT NULL
        );
    "#;

    sqlx::query(query).execute(pool).await?;
    Ok(())
}

pub async fn create_produtos(pool: &PgPool) -> Result<(), sqlx::Error> {
    let create_produtos = r#"
        CREATE TABLE IF NOT EXISTS produtos (
            id SERIAL PRIMARY KEY,
            loja_id INT NOT NULL REFERENCES lojas(id),
            nome VARCHAR(100) NOT NULL,
            marca VARCHAR(50),
            cor VARCHAR(30),
            valor FLOAT NOT NULL,
            desconto FLOAT DEFAULT 0,
            descricao VARCHAR(1000),
            categoria_id INT REFERENCES categorias(id),
            frete BOOLEAN DEFAULT FALSE,
            outlet BOOLEAN DEFAULT FALSE,
            ativo BOOLEAN DEFAULT TRUE,
            criado_em TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            registrado_por INT REFERENCES usuarios(id)
        );
    "#;

    sqlx::query(create_produtos).execute(pool).await?;
    Ok(())
}

pub async fn create_cartegorias(pool: &PgPool) -> Result<(), sqlx::Error> {
    let create_categorias = r#"
        CREATE TABLE IF NOT EXISTS categorias (
            id SERIAL PRIMARY KEY,
            nome VARCHAR(50) UNIQUE NOT NULL
        );
    "#;

    sqlx::query(create_categorias).execute(pool).await?;
    Ok(())
}

pub async fn create_cupons(pool: &PgPool) -> Result<(), sqlx::Error> {
    let create_cupons = r#"
    CREATE TABLE IF NOT EXISTS cupons (
            id SERIAL PRIMARY KEY,
            codigo VARCHAR(50) UNIQUE NOT NULL
        );
    "#;

    sqlx::query(create_cupons).execute(pool).await?;
    Ok(())
}

pub async fn create_produtos_cupons(pool: &PgPool) -> Result<(), sqlx::Error> {
    let create_produtos_cupons = r#"
    CREATE TABLE IF NOT EXISTS produto_cupons (
            produto_id INT NOT NULL REFERENCES produtos(id),
            cupom_id INT NOT NULL REFERENCES cupons(id),
            PRIMARY KEY (produto_id, cupom_id)
        );
    "#;

    sqlx::query(create_produtos_cupons).execute(pool).await?;
    Ok(())
}

pub async fn create_imagens(pool: &PgPool) -> Result<(), sqlx::Error> {
    let create_imagens = r#"CREATE TABLE IF NOT EXISTS imagens (
            id SERIAL PRIMARY KEY,
            produto_id INT NOT NULL REFERENCES produtos(id),
            url TEXT NOT NULL
        );
    "#;

    sqlx::query(create_imagens).execute(pool).await?;
    Ok(())
}

pub async fn create_all_tables(pool: &PgPool) {
    create_lojas(pool).await.expect("Failed to create lojas table");
    create_usuarios(pool).await.expect("Failed to create usuarios table");
    create_cartegorias(pool).await.expect("Failed to create categorias table");
    create_produtos(pool).await.expect("Failed to create produtos table");
    create_cupons(pool).await.expect("Failed to create cupons table");
    create_produtos_cupons(pool).await.expect("Failed to create produto_cupons table");
    create_imagens(pool).await.expect("Failed to create imagens table");
}