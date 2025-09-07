use actix_web::web;
use crate::controllers::product_controller::*;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::post().to(create_product))
            .route("", web::get().to(get_products))
            .route("/{id}", web::get().to(get_product_by_id))
            .route("/{id}", web::put().to(update_product))
            .route("/{id}", web::delete().to(delete_product)),
    );
}
