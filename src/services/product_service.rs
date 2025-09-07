use crate::models::product::{CreateProduct, Product, UpdateProduct};
use sqlx::MySqlPool;
use uuid::Uuid;

pub async fn create_product(
    pool: &MySqlPool,
    new_product: CreateProduct,
) -> Result<Product, sqlx::Error> {
    let id = Uuid::new_v4().to_string();

    sqlx::query!(
        "INSERT INTO products (id, name, description, price) VALUES (?, ?, ?, ?)",
        id,
        new_product.name,
        new_product.description,
        new_product.price
    )
    .execute(pool)
    .await?;

    // Fetch the product you just inserted:
    let rec = sqlx::query_as!(
        Product,
        "SELECT id, name, description, price, created_at FROM products WHERE id = ?",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

pub async fn get_all_products(pool: &MySqlPool) -> Result<Vec<Product>, sqlx::Error> {
    let products = sqlx::query_as!(Product, "SELECT * FROM products")
        .fetch_all(pool)
        .await?;
    Ok(products)
}

pub async fn get_product_by_id(pool: &MySqlPool, id: &str) -> Result<Option<Product>, sqlx::Error> {
    let product = sqlx::query_as!(Product, "SELECT * FROM products WHERE id = ?", id)
        .fetch_optional(pool)
        .await?;
    Ok(product)
}

pub async fn update_product(
    pool: &MySqlPool,
    id: &str,
    product: UpdateProduct,
) -> Result<Option<Product>, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE products SET name = COALESCE(?, name), description = COALESCE(?, description), price = COALESCE(?, price) WHERE id = ?",
        product.name,
        product.description,
        product.price,
        id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Ok(None);
    }

    let updated = sqlx::query_as!(
        Product,
            "SELECT id, name, description, price, created_at FROM products WHERE id = ?",
            id
    )
    .fetch_optional(pool)
    .await?;

    Ok(updated)
}

pub async fn delete_product(pool: &MySqlPool, id: &str) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM products WHERE id = ?", id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
