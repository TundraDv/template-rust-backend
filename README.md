# Rust Backend Template

A production-ready Rust backend template built with Axum, SeaORM, and PostgreSQL, featuring multi-tenancy, JWT authentication, and comprehensive error handling.

## Table of Contents

- [Features](#features)
- [Configuration](#configuration)
- [Error Structure](#error-structure)
- [API Endpoints](#api-endpoints)
- [Setup](#setup)
- [Running](#running)
- [Testing](#testing)

## Features

- **Multi-tenancy**: Tenant isolation with access control
- **JWT Authentication**: Secure token-based authentication
- **Role-Based Access Control**: Admin and Regular user roles
- **Type-Safe Error Handling**: Comprehensive error system with consistent responses
- **Environment-Aware CORS**: Development and production configurations
- **Health Check**: Database connectivity monitoring
- **Structured Logging**: Request tracing with unique IDs
- **Password Security**: Argon2 password hashing

## Configuration

The application uses environment variables for configuration. Create a `.env` file in the root directory:

### Required Variables

```bash
DATABASE_URL=postgresql://user:password@localhost:5432/dbname
```

### Optional Variables

```bash
# Server Configuration
SERVER_HOST=0.0.0.0          # Default: 0.0.0.0
SERVER_PORT=8070            # Default: 8070

# JWT Configuration
BEARER_TOKEN=your-secret-key-change-in-production  # Required for production
JWT_EXPIRATION_MINUTES=10   # Default: 10

# Environment
ENVIRONMENT=dev             # Options: dev, development, prod, production (Default: production)

# CORS Configuration (Required in production)
FRONTEND_URL=https://your-frontend.com  # Required when ENVIRONMENT=production
```

### Configuration Details

- **DATABASE_URL**: PostgreSQL connection string
- **SERVER_HOST**: Host to bind the server (default: `0.0.0.0`)
- **SERVER_PORT**: Port to bind the server (default: `8070`)
- **BEARER_TOKEN**: Secret key for JWT signing (defaults to insecure value - change in production)
- **JWT_EXPIRATION_MINUTES**: Token expiration time in minutes (default: `10`)
- **ENVIRONMENT**: Environment mode
  - `dev` or `development`: Allows all CORS origins
  - `prod` or `production`: Restricts CORS to `FRONTEND_URL`
- **FRONTEND_URL**: Frontend URL for CORS in production (required when `ENVIRONMENT=production`)

## Error Structure

All API errors follow a consistent structure:

```json
{
  "error": "ERROR_CODE",
  "message": "Human-readable error message"
}
```

### Error Codes

| Error Code | HTTP Status | Description |
|------------|-------------|-------------|
| `TOKEN_EXPIRED` | 401 | JWT token has expired |
| `INVALID_TOKEN` | 401 | JWT token is invalid or malformed |
| `MISSING_TOKEN` | 401 | Authorization header is missing |
| `INVALID_CREDENTIALS` | 401 | Email or password is incorrect |
| `USER_NOT_FOUND` | 404 | User does not exist |
| `USER_ALREADY_EXISTS` | 409 | User already exists for the tenant |
| `USER_NOT_VALIDATED` | 403 | User account is not active |
| `TENANT_NOT_FOUND` | 404 | Tenant does not exist |
| `FORBIDDEN` | 403 | Access denied (with custom message) |
| `DATABASE_ERROR` | 500 | Database operation failed |
| `INTERNAL_ERROR` | 500 | Internal server error |
| `SERVICE_UNAVAILABLE` | 503 | Service is currently unavailable |

### Error Response Example

```json
{
  "error": "USER_NOT_FOUND",
  "message": "User not found"
}
```

## API Endpoints

### Public Endpoints

#### Health Check

```http
GET /health
```

Check service and database health.

**Response:**
```json
{
  "status": "healthy",
  "database": "connected"
}
```

**Error Responses:**
- `503 SERVICE_UNAVAILABLE`: Database is disconnected

---

#### Register User

```http
POST /api/auth/register
Authorization: Bearer <BEARER_TOKEN>
Content-Type: application/json
```

Register a new user. First user for a tenant becomes Admin, subsequent users are Regular.

**Request Body:**
```json
{
  "tenant_id": "uuid",
  "email": "user@example.com",
  "password": "securepassword"
}
```

**Response:**
```json
{
  "token": "jwt_token_string",
  "user": {
    "id": "uuid",
    "tenant_id": "uuid",
    "email": "user@example.com",
    "role": "admin",
    "status": "active",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

**Error Responses:**
- `409 USER_ALREADY_EXISTS`: User already exists for this tenant
- `500 DATABASE_ERROR`: Database operation failed
- `500 INTERNAL_ERROR`: Internal server error

---

#### Login

```http
POST /api/auth/login
Authorization: Bearer <BEARER_TOKEN>
Content-Type: application/json
```

Authenticate user and receive JWT token.

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "securepassword"
}
```

**Response:**
```json
{
  "token": "jwt_token_string",
  "user": {
    "id": "uuid",
    "tenant_id": "uuid",
    "email": "user@example.com",
    "role": "admin",
    "status": "active",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

**Error Responses:**
- `401 INVALID_CREDENTIALS`: Email or password is incorrect
- `403 USER_NOT_VALIDATED`: User account is not active
- `500 DATABASE_ERROR`: Database operation failed
- `500 INTERNAL_ERROR`: Internal server error

---

#### List Tenants

```http
GET /api/tenants
```

Get list of all tenants.

**Response:**
```json
[
  {
    "id": "uuid",
    "name": "Tenant Name",
    "status": "active",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
]
```

**Error Responses:**
- `500 DATABASE_ERROR`: Database operation failed

---

### Authenticated Endpoints

All authenticated endpoints require a valid JWT token in the Authorization header:

```http
Authorization: Bearer <JWT_TOKEN>
```

#### Refresh Token

```http
POST /api/auth/refresh
Authorization: Bearer <JWT_TOKEN>
```

Refresh the JWT token.

**Response:**
```json
{
  "token": "new_jwt_token_string",
  "user": {
    "id": "uuid",
    "tenant_id": "uuid",
    "email": "user@example.com",
    "role": "admin",
    "status": "active",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

**Error Responses:**
- `401 TOKEN_EXPIRED`: Token has expired
- `401 INVALID_TOKEN`: Token is invalid
- `401 MISSING_TOKEN`: Authorization header missing
- `404 USER_NOT_FOUND`: User not found
- `403 USER_NOT_VALIDATED`: User account is not active
- `500 DATABASE_ERROR`: Database operation failed
- `500 INTERNAL_ERROR`: Internal server error

---

#### Get Current User

```http
GET /api/me
Authorization: Bearer <JWT_TOKEN>
```

Get the current authenticated user's information.

**Response:**
```json
{
  "id": "uuid",
  "email": "user@example.com",
  "tenant_id": "uuid",
  "status": "active",
  "role": "admin",
  "created_at": "2024-01-01T00:00:00Z"
}
```

**Error Responses:**
- `401 TOKEN_EXPIRED`: Token has expired
- `401 INVALID_TOKEN`: Token is invalid
- `401 MISSING_TOKEN`: Authorization header missing
- `404 USER_NOT_FOUND`: User not found
- `500 DATABASE_ERROR`: Database operation failed

---

#### Get Tenant

```http
GET /api/tenants/{tenant_id}
Authorization: Bearer <JWT_TOKEN>
```

Get tenant information. User must belong to the specified tenant.

**Path Parameters:**
- `tenant_id` (UUID): Tenant identifier

**Response:**
```json
{
  "id": "uuid",
  "name": "Tenant Name",
  "status": "active",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

**Error Responses:**
- `401 TOKEN_EXPIRED`: Token has expired
- `401 INVALID_TOKEN`: Token is invalid
- `401 MISSING_TOKEN`: Authorization header missing
- `403 FORBIDDEN`: User does not belong to this tenant
- `404 TENANT_NOT_FOUND`: Tenant not found
- `500 DATABASE_ERROR`: Database operation failed

---

#### Get User

```http
GET /api/tenants/{tenant_id}/users/{user_id}
Authorization: Bearer <JWT_TOKEN>
```

Get user information. User must belong to the specified tenant.

**Path Parameters:**
- `tenant_id` (UUID): Tenant identifier
- `user_id` (UUID): User identifier

**Response:**
```json
{
  "id": "uuid",
  "tenant_id": "uuid",
  "email": "user@example.com",
  "role": "admin",
  "status": "active",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

**Error Responses:**
- `401 TOKEN_EXPIRED`: Token has expired
- `401 INVALID_TOKEN`: Token is invalid
- `401 MISSING_TOKEN`: Authorization header missing
- `403 FORBIDDEN`: User does not belong to this tenant
- `404 USER_NOT_FOUND`: User not found
- `500 DATABASE_ERROR`: Database operation failed

---

### Admin Endpoints

Admin endpoints require:
1. Valid JWT token
2. Admin role
3. User must belong to the specified tenant

#### List Users

```http
GET /api/tenants/{tenant_id}/users
Authorization: Bearer <JWT_TOKEN>
```

Get list of all users for a tenant. Requires Admin role.

**Path Parameters:**
- `tenant_id` (UUID): Tenant identifier

**Response:**
```json
[
  {
    "id": "uuid",
    "tenant_id": "uuid",
    "email": "user@example.com",
    "role": "admin",
    "status": "active",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
]
```

**Error Responses:**
- `401 TOKEN_EXPIRED`: Token has expired
- `401 INVALID_TOKEN`: Token is invalid
- `401 MISSING_TOKEN`: Authorization header missing
- `403 FORBIDDEN`: Admin role required or user does not belong to this tenant
- `500 DATABASE_ERROR`: Database operation failed

---

#### Change User Status

```http
PUT /api/tenants/{tenant_id}/users/{user_id}/change-status
Authorization: Bearer <JWT_TOKEN>
```

Toggle user status between Active and Inactive. Requires Admin role.

**Path Parameters:**
- `tenant_id` (UUID): Tenant identifier
- `user_id` (UUID): User identifier

**Response:**
```json
{
  "id": "uuid",
  "email": "user@example.com",
  "status": "inactive",
  "message": "User status changed to Inactive successfully"
}
```

**Error Responses:**
- `401 TOKEN_EXPIRED`: Token has expired
- `401 INVALID_TOKEN`: Token is invalid
- `401 MISSING_TOKEN`: Authorization header missing
- `403 FORBIDDEN`: Admin role required or user does not belong to this tenant
- `404 USER_NOT_FOUND`: User not found
- `500 DATABASE_ERROR`: Database operation failed

---

#### Change User Role

```http
PUT /api/tenants/{tenant_id}/users/{user_id}/change-role
Authorization: Bearer <JWT_TOKEN>
```

Toggle user role between Admin and Regular. Requires Admin role.

**Path Parameters:**
- `tenant_id` (UUID): Tenant identifier
- `user_id` (UUID): User identifier

**Response:**
```json
{
  "id": "uuid",
  "email": "user@example.com",
  "role": "regular",
  "message": "User role changed to Regular successfully"
}
```

**Error Responses:**
- `401 TOKEN_EXPIRED`: Token has expired
- `401 INVALID_TOKEN`: Token is invalid
- `401 MISSING_TOKEN`: Authorization header missing
- `403 FORBIDDEN`: Admin role required or user does not belong to this tenant
- `404 USER_NOT_FOUND`: User not found
- `500 DATABASE_ERROR`: Database operation failed

---

## Setup

### Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Cargo

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd template-rust-backend
```

2. Create a `.env` file:
```bash
cp .env.example .env
# Edit .env with your configuration
```

3. Run database migrations:
```bash
cargo run -- run_migrations
```

4. Build the project:
```bash
cargo build --release
```

## Running

### Development

```bash
cargo run
```

### Production

```bash
cargo run --release
```

The server will start on `http://SERVER_HOST:SERVER_PORT` (default: `http://0.0.0.0:8070`).

### Running Migrations

```bash
cargo run -- run_migrations
```

## Testing

The project includes comprehensive unit and integration tests.

### Test Structure

```
tests/
├── auth_service_test.rs      # Unit tests for authentication service
├── validation_test.rs         # Unit tests for request validation
├── common/                    # Shared test utilities
│   └── mod.rs
└── integration/               # Integration tests for API endpoints
    ├── auth.rs                # Authentication endpoint tests
    ├── health.rs              # Health check endpoint tests
    ├── users.rs               # User management endpoint tests
    └── tenants.rs             # Tenant endpoint tests
```

### Running Tests

#### Run All Tests

```bash
cargo test
```

#### Run Only Unit Tests

```bash
cargo test --test auth_service_test
cargo test --test validation_test
```

#### Run Only Integration Tests

```bash
cargo test --test integration
```

#### Run Tests with Output

```bash
cargo test -- --nocapture
```

#### Run Specific Test

```bash
cargo test test_hash_password
```

### Unit Tests

Unit tests are located in the `tests/` directory and test individual functions and services:

- **`auth_service_test.rs`**: Tests for password hashing, JWT token generation/verification
- **`validation_test.rs`**: Tests for request validation (email format, password length, etc.)

### Integration Tests

Integration tests are located in `tests/integration/` and test full HTTP endpoints:

- **`auth.rs`**: Tests for `/api/auth/register`, `/api/auth/login`, `/api/auth/refresh`
- **`health.rs`**: Tests for `/health` endpoint
- **`users.rs`**: Tests for user management endpoints
- **`tenants.rs`**: Tests for tenant endpoints

**Note**: Integration tests require a test database. Tests are currently marked with `#[ignore]` until test database setup is configured. To run ignored tests:

```bash
cargo test -- --ignored
```

### Test Coverage

Current test coverage includes:

- ✅ Password hashing and verification
- ✅ JWT token generation and validation
- ✅ Request validation (email, password length)
- ✅ Error handling
- ⏳ API endpoint integration tests (require test database setup)

### Adding New Tests

1. **Unit Tests**: Create a new file in `tests/` directory (e.g., `tests/my_service_test.rs`)
2. **Integration Tests**: Add to appropriate file in `tests/integration/` or create new file

Example unit test:

```rust
use template_rust_backend::services::my_service::MyService;

#[test]
fn test_my_function() {
    let result = MyService::my_function();
    assert!(result.is_ok());
}
```

## Data Models

### User Model

```rust
{
  "id": "uuid",
  "tenant_id": "uuid",
  "email": "string",
  "role": "admin" | "regular",
  "status": "active" | "inactive",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

### Tenant Model

```rust
{
  "id": "uuid",
  "name": "string",
  "status": "active" | "inactive",
  "created_at": "datetime",
  "updated_at": "datetime"
}
```

## Authentication

### JWT Token Structure

JWT tokens contain the following claims:

```json
{
  "user_id": "uuid",
  "tenant_id": "uuid",
  "email": "string",
  "role": "admin" | "regular",
  "exp": 1234567890
}
```

### Using JWT Tokens

Include the token in the Authorization header:

```http
Authorization: Bearer <JWT_TOKEN>
```

Tokens expire after the time specified in `JWT_EXPIRATION_MINUTES` (default: 10 minutes). Use the refresh endpoint to obtain a new token.

## License

[Your License Here]

