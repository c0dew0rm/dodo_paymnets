# Dodo Payments Backend Service

## Setup and Run

### Prerequisites

- Rust
- Docker
- Docker Compose
- Postgres

### Setting up Postgres
1. Install postgres on you OS, in case it is not installed.
2. Run the following set of commands after installation

```plaintext 
CREATE DATABASE dodo_payments;
CREATE USER dodouser WITH PASSWORD 'password';
ALTER ROLE dodouser SET client_encoding TO 'utf8';
ALTER ROLE dodouser SET default_transaction_isolation TO 'read committed';
ALTER ROLE dodouser SET timezone TO 'UTC';
ALTER USER dodouser WITH SUPERUSER;
GRANT ALL PRIVILEGES ON DATABASE dodo_payments TO dodouser;

\c dodo_payments

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    balance DOUBLE PRECISION NOT NULL DEFAULT 0.0
);

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

### Running with Docker

1. Clone the repository.
2. Create a `.env` file with the following content:

```plaintext
DATABASE_URL=postgres://postgres://dodouser:password@localhost:5432/dodo_payments
SECRET_KEY="abcd"
```

### Running locally
1. Intall rust.
2. Go inside project directory.
3. Run the following commands
```plaintext
cargo build
cargo run
```