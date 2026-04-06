# 💼 Zorvyn Finance Backend

A role-based finance backend built using **Rust + Axum + PostgreSQL**, designed to demonstrate clean backend architecture, secure access control, and meaningful data processing.

This project focuses on **clarity, correctness, and real-world backend patterns** rather than unnecessary abstraction.

---

## 🎯 Problem Context

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


## 📂 Project Structure

```text id="7j7hzr"
src/
├── config/
├── dto/
├── handlers/
├── middleware/
├── models/
├── repositories/
├── routes/
├── utils/
```

---

## ⚙️ Tech Stack

```toml id="gmbf6d"
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

---

## 🧠 Architecture Overview

```text id="ztmru3"
Routes → Handlers → Repositories → Database
           ↓
        Middleware (Auth + RBAC)
           ↓
           Utils (JWT, Hashing, Errors)
```

---

## 🔐 Authentication & RBAC

### Authentication

* JWT-based authentication using `jsonwebtoken`
* Claims include:

  * `sub` (user_id)
  * `email`
  * `role`

---

# 🔑 Role Capabilities

---

## 👀 Viewer

```text id="ah1j52"
User API:
✔ View own profile

Transaction API:
✔ View transactions
❌ Cannot create/update/delete

Dashboard API:
✔ View dashboard
```

---

## 📊 Analyst

```text id="cifqkk"
User API:
✔ View own profile
❌ Cannot manage users

Transaction API:
✔ Create transactions
✔ Update own transactions
✔ Delete own transactions
✔ View transactions
❌ Cannot modify others' data

Dashboard API:
✔ Full access
```

---

## 👑 Admin

```text id="9h4o3f"
User API:
✔ List users
✔ Update roles
✔ Delete users

Transaction API:
✔ Full CRUD
✔ Can delete ANY transaction (override)

Dashboard API:
✔ Full access
```

---

## 💰 Transaction System

```text id="8n4i9l"
amount | type | category | created_at | description | user_id
```

* Ownership enforced at DB level
* Admin override supported
* QueryBuilder used for dynamic filtering

---

## 🔍 Filtering & Pagination

```text id="m1r2q6"
/type=income|expense
/category=food
/start_date=...
/end_date=...
/limit=10
/offset=0
```

---

## 📊 Dashboard

* Total income
* Total expense
* Net balance
* Category-wise totals
* Recent activity
* Monthly trends

---

# ⚙️ Setup & Run

---

## 1️⃣ Clone Repository

```bash id="q78t7k"
git clone <repo>
cd zorvyn-backend
```

---

## 2️⃣ Setup Environment

Create `.env` file:

```env id="n1lygh"
DATABASE_URL=postgres://user:password@localhost/zorvyn_db
JWT_SECRET=your_secret_key
```

---

## 3️⃣ Create Database

```bash id="9k7a5s"
createdb zorvyn_db
```

---

## 4️⃣ Run Migrations (SQLx)

If using SQLx CLI:

```bash id="d8m9rg"
sqlx database create
sqlx migrate run
```

> Ensure you have SQLx CLI installed:

```bash id="dfn5g4"
cargo install sqlx-cli --no-default-features --features postgres
```

---

## 5️⃣ Seed Database

```bash id="6j1p9c"
psql $DATABASE_URL -f db/seed.sql
```

This will populate:

* Users (viewer, analyst, admin)
* Transactions across categories and dates

---

## 6️⃣ Run Server

```bash id="ljj8sk"
cargo run
```

Server runs at:

```text id="o0pq6r"
http://127.0.0.1:3000
```

---

## 🧪 Example Request

```http id="ch7l7o"
GET /transactions?limit=5&offset=0
Authorization: Bearer <token>
```

---

## 🌱 Seed Data Notes

* Includes realistic financial data
* Covers multiple categories
* Time-distributed for trend testing
* Useful for testing dashboard + filters

---

## 🔐 Security

* Argon2 password hashing
* JWT-based auth
* RBAC via middleware
* Ownership enforced in queries

---

