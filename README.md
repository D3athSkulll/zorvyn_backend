#  Zorvyn Finance Backend

A role-based finance backend built using **Rust + Axum + PostgreSQL**, designed to demonstrate clean backend architecture, secure access control, and meaningful data processing.

This project focuses on **clarity, correctness, and real-world backend patterns** rather than unnecessary abstraction.

---

##  Problem Context

This backend was built for a **finance dashboard system** where:

* Multiple users interact with financial data
* Each user has a **role with restricted access**
* The system supports:

  * Transaction management
  * Role-based permissions
  * Aggregated financial insights

The goal was to build a system that is:

* **Logically sound**
* **Secure**
* **Maintainable**

---

##  Tech Stack

```toml
axum = "0.8.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "uuid", "chrono", "macros"] }
dotenvy = "0.15"
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
argon2 = "0.5"
rand = "0.8"
jsonwebtoken = "9"
validator = { version = "0.18", features = ["derive"] }
```

### Why these choices?

* **Axum** → lightweight, type-safe routing
* **SQLx** → compile-time checked queries (no ORM overhead)
* **Argon2** → secure password hashing
* **jsonwebtoken** → stateless auth
* **validator** → declarative input validation
* **chrono + uuid** → production-grade data types

---

## Architecture Overview

The project follows a **layered architecture**:

```text
Routes → Handlers → Repositories → Database
           ↓
        Middleware (Auth + RBAC)
           ↓
           Utils (JWT, Hashing, Errors)
```

### Key Design Decisions

* **No heavy frameworks/ORMs** → full control over queries
* **Separation of concerns** → handlers don’t contain DB logic
* **Middleware-driven auth** → keeps business logic clean
* **Consistent response format** → predictable API contract

---

## Authentication & RBAC

### Authentication

* JWT-based authentication using `jsonwebtoken`
* Claims include:

  * `sub` (user_id)
  * `email`
  * `role`

### Authorization (RBAC)

| Role    | Access                                |
| ------- | --------------------------------------|
| Viewer  | Read-only                             |
| Analyst | Read + Create + Update + Delete (Own) |
| Admin   | Full access                           |

RBAC is enforced via **custom middleware**, not inside handlers.

---

## Transaction System

Each transaction includes:

```text
amount | type | category | created_at | description | user_id
```

### Supported Operations

* Create transaction
* Fetch transactions (with filters + pagination)
* Update transaction (ownership enforced)
* Delete transaction (RBAC enforced)

---

## Filtering & Pagination

Supports dynamic filtering using **SQLx QueryBuilder**:

```text
/type=income|expense
/category=food
/start_date=...
/end_date=...
```

Pagination:

```text
GET /transactions?limit=10&offset=0
```

This prevents large data fetches and improves performance.

---

## Dashboard API

The `/dashboard` endpoint provides:

### Aggregations

* Total income
* Total expense
* Net balance

### Category Breakdown

* Grouped totals per category

### Recent Activity

* Last 5 transactions

### Monthly Trends

* Aggregation using `DATE_TRUNC('month', created_at)`

---

## Admin User Management

(Admin-only endpoints)

* List all users
* Update user role
* Delete user

Sensitive fields like `password_hash` are **never exposed**.

---

## Validation & Error Handling

### Validation

* Implemented using `validator`
* Applied at request boundary (DTO layer)

### Error Handling

* Centralized using custom `AppError`
* Consistent JSON responses:

```json
{
  "success": false,
  "message": "Error message"
}
```

Validation errors return structured field-level feedback.

---

## Security Considerations

* Passwords hashed using **Argon2**
* JWT secret stored in `.env`
* User ownership enforced in queries:

  ```sql
  WHERE id = $1 AND user_id = $2
  ```
* RBAC enforced via middleware

---

## Project Structure

```text
src/
├── config/        # App state, DB config
├── dto/           # Request/response structs
├── handlers/      # Route handlers
├── middleware/    # Auth + RBAC
├── models/        # Models for User and Transaction
├── repositories/  # DB queries
├── routes/        # Route definitions
├── utils/         # JWT, hashing, error handling

```

---

## Setup & Run

### 1. Clone

```bash
git clone <repo>
cd zorvyn-backend
```

---

### 2. Environment

Create `.env`:

```env
DATABASE_URL=postgres://user:password@localhost/zorvyn_db
JWT_SECRET=your_secret_key
```

---

### 3. Run

```bash
cargo run
```

Server:

```text
http://127.0.0.1:3000
```

---

## Example

```http
GET /transactions?type=expense&limit=5&offset=0
Authorization: Bearer <token>
```

---

## Trade-offs & Scope Decisions

* No logging system integrated (kept scope focused)
* No Swagger/OpenAPI (manual testing assumed)
* No background jobs or async workers
* Focus was on **correctness over feature overload**

---
