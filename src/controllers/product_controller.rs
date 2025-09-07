use actix_web::{web, HttpResponse};
use crate::{models::product::*, services::product_service};
use sqlx::MySqlPool;

pub async fn create_product(
    pool: web::Data<MySqlPool>,
    item: web::Json<CreateProduct>,
) -> HttpResponse {
    match product_service::create_product(&pool, item.into_inner()).await {
        Ok(prod) => HttpResponse::Ok().json(prod),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_products(pool: web::Data<MySqlPool>) -> HttpResponse {
    match product_service::get_all_products(&pool).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_product_by_id(
    pool: web::Data<MySqlPool>,
    id: web::Path<String>,
) -> HttpResponse {
    match product_service::get_product_by_id(&pool, &id).await {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update_product(
    pool: web::Data<MySqlPool>,
    id: web::Path<String>,
    item: web::Json<UpdateProduct>,
) -> HttpResponse {
    match product_service::update_product(&pool, &id, item.into_inner()).await {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_product(
    pool: web::Data<MySqlPool>,
    id: web::Path<String>,
) -> HttpResponse {
    match product_service::delete_product(&pool, &id).await {
        Ok(rows) if rows > 0 => HttpResponse::Ok().body("Deleted"),
        Ok(_) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
