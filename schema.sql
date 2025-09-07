-- E-commerce Product API Database Schema
-- MySQL Database Schema for Product Management

-- Create database (run this first)
-- CREATE DATABASE IF NOT EXISTS ecommerce;
-- USE ecommerce;

-- Products table
CREATE TABLE IF NOT EXISTS products (
    id VARCHAR(36) PRIMARY KEY COMMENT 'UUID v4 identifier',
    name VARCHAR(255) NOT NULL COMMENT 'Product name',
    description TEXT COMMENT 'Product description',
    price DECIMAL(10,2) COMMENT 'Product price with 2 decimal places',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT 'Record creation timestamp',
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT 'Record last update timestamp'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Add indexes for better performance
CREATE INDEX idx_products_name ON products(name);
CREATE INDEX idx_products_created_at ON products(created_at);

-- Sample data (optional - for testing)
-- INSERT INTO products (id, name, description, price) VALUES
-- ('550e8400-e29b-41d4-a716-446655440000', 'Laptop', 'High-performance laptop for gaming and work', 999.99),
-- ('550e8400-e29b-41d4-a716-446655440001', 'Mouse', 'Wireless optical mouse', 29.99),
-- ('550e8400-e29b-41d4-a716-446655440002', 'Keyboard', 'Mechanical gaming keyboard', 149.99);
