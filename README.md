# E-commerce Product API

A RESTful API built with Rust using Actix-Web framework for managing products in an e-commerce system.

## Features

- **CRUD Operations**: Create, Read, Update, and Delete products
- **MySQL Database**: Persistent storage with SQLx for database operations
- **JSON API**: RESTful endpoints with JSON request/response
- **Environment Configuration**: Configurable via environment variables
- **Async/Await**: Built with modern async Rust patterns

## Tech Stack

- **Framework**: Actix-Web 4.11.0
- **Database**: MySQL with SQLx 0.8.6
- **Serialization**: Serde for JSON handling
- **UUID**: UUID v4 for unique product identifiers
- **Environment**: dotenv for configuration management

## Project Structure

```
src/
├── main.rs              # Application entry point
├── config/
│   ├── mod.rs
│   └── db.rs           # Database configuration
├── models/
│   ├── mod.rs
│   └── product.rs      # Product data models
├── controllers/
│   ├── mod.rs
│   └── product_controller.rs  # HTTP request handlers
├── services/
│   ├── mod.rs
│   └── product_service.rs     # Business logic
└── routes/
    ├── mod.rs
    └── product_routes.rs      # Route definitions
```

## API Endpoints

### Products

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/products` | Create a new product |
| GET | `/products` | Get all products |
| GET | `/products/{id}` | Get product by ID |
| PUT | `/products/{id}` | Update product by ID |
| DELETE | `/products/{id}` | Delete product by ID |

### Request/Response Examples

#### Create Product
```bash
POST /products
Content-Type: application/json

{
  "name": "Laptop",
  "description": "High-performance laptop",
  "price": "999.99"
}
```

#### Response
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Laptop",
  "description": "High-performance laptop",
  "price": "999.99",
  "created_at": "2024-01-01T12:00:00"
}
```

#### Update Product
```bash
PUT /products/{id}
Content-Type: application/json

{
  "name": "Gaming Laptop",
  "price": "1299.99"
}
```

## Database Schema

```sql
CREATE TABLE products (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price DECIMAL(10,2),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## Prerequisites

- Rust 1.70+ (2024 edition)
- MySQL 8.0+
- Cargo package manager

## Installation & Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd product_api
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Database Setup**
   - Create a MySQL database named `ecommerce`
   - Run the database schema (see Database Schema section)

4. **Environment Configuration**
   - Copy `.env.example` to `.env`
   - Update the environment variables with your configuration

5. **Run the application**
   ```bash
   cargo run
   ```

The server will start on `http://127.0.0.1:5000` (or configured HOST:PORT)

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `HOST` | Server host address | `127.0.0.1` |
| `PORT` | Server port | `8080` |
| `DATABASE_URL` | MySQL connection string | Required |

## Development

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

### Building for Production
```bash
cargo build --release
```

## API Testing

You can test the API using curl, Postman, or any HTTP client:

```bash
# Create a product
curl -X POST http://localhost:5000/products \
  -H "Content-Type: application/json" \
  -d '{"name":"Test Product","description":"A test product","price":"29.99"}'

# Get all products
curl http://localhost:5000/products

# Get specific product
curl http://localhost:5000/products/{product-id}

# Update product
curl -X PUT http://localhost:5000/products/{product-id} \
  -H "Content-Type: application/json" \
  -d '{"name":"Updated Product","price":"39.99"}'

# Delete product
curl -X DELETE http://localhost:5000/products/{product-id}
```

## Security

- Never commit `.env` files with sensitive information
- Use environment variables for all configuration
- Validate all input data
- Use prepared statements (SQLx handles this automatically)

## Troubleshooting

### Common Issues

1. **Database Connection Failed**
   - Verify MySQL is running
   - Check DATABASE_URL format
   - Ensure database exists

2. **Port Already in Use**
   - Change PORT in .env file
   - Kill existing process on the port

3. **Compilation Errors**
   - Update Rust to latest version
   - Clear cargo cache: `cargo clean`

## Support

For support and questions, please open an issue in the repository.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📬 Contact

For questions, feedback, or contributions, feel free to reach out:

* **Developer:** Roshan Jha
* **Email:** [roshanjha.work@gmail.com](mailto:roshanjha.work@gmail.com)
* **GitHub:** [roshanjha0412](https://github.com/roshanjha0412)
