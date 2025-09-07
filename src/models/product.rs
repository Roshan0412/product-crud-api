use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<BigDecimal>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: Option<BigDecimal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<BigDecimal>,
}