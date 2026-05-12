# Konnect

> *"A service that cannot reach its data is a service that does not exist."*
> — Komrad Engineering Collective, May 2026

Konnect is the PostgreSQL connection management library of the Komrad ecosystem. It provides a typed configuration, a connection pool initializer, and a `Store` trait that all persistence layers across the stack implement. Consumed by [Korelator](https://github.com/komrad-company/Korelator) and any other Komrad component that requires database access.

Konnect does not query. Konnect does not define schemas. Konnect **connects**. The tables, the queries, the migrations — those belong to the consumer.

```
DatabaseConfig ──init()──► PgPool ──► AlertStore, PipelineStore, ... (defined by consumers)
```

---

## Configuration

Each consuming service declares a `DatabaseConfig` in its own configuration file.

```json
{
  "database": {
    "host": "localhost",
    "port": 5432,
    "database": "komrad",
    "user": "korelator",
    "password": "...",
    "schema": "korelator",
    "search_path": "korelator"
  }
}
```

| Field | Type | Purpose |
|---|---|---|
| `host` | string | PostgreSQL host |
| `port` | u16 | PostgreSQL port |
| `database` | string | Target database name |
| `user` | string | PostgreSQL role used by this service |
| `password` | string | Credential for the role |
| `schema` | string | Schema owned by this service |
| `search_path` | string | PostgreSQL `search_path` — set on every connection |

The `search_path` is injected into the connection URL. No connection opened through Konnect can silently resolve a table outside the declared schema.

---

## API

```rust
use konnect::{init, DatabaseConfig, Store};

// Initialise the pool — call once at startup
let pool = konnect::init(&config.database).await?;
```

Consumers implement the `Store` trait on their own store types:

```rust
pub struct AlertStore {
    pool: konnect::PgPool,
}

impl konnect::Store for AlertStore {
    fn new(pool: konnect::PgPool) -> Self {
        Self { pool }
    }

    fn pool(&self) -> &konnect::PgPool {
        &self.pool
    }

    async fn migrate(&self) -> Result<(), konnect::Error> {
        sqlx::migrate!("./migrations")
            .run(self.pool())
            .await
            .map_err(konnect::Error::MigrationError)
    }
}
```

`migrate` embeds SQL migration files at compile time via `sqlx::migrate!()`. An invalid migration file fails the build.

### Public types

| Type | Role |
|---|---|
| `DatabaseConfig` | Full connection configuration for one service |
| `PgPool` | Re-exported `sqlx::PgPool` — consumers need not depend on `sqlx` directly for the pool type |
| `Store` | Trait to implement on any store struct — enforces `new`, `pool`, and `migrate` |
| `Error` | Connection and migration errors — the caller must handle them |

### Error variants

| Variant | Source |
|---|---|
| `ConnectionError` | `sqlx::Error` — pool creation failed |
| `MigrationError` | `sqlx::migrate::MigrateError` — migration failed at startup |
| `InvalidConfiguration` | Invalid configuration field — produced by the consumer |

---

## Dependencies

| Crate | Purpose |
|---|---|
| `sqlx` | Async PostgreSQL driver, connection pool, migrations |
| `serde` | Configuration deserialization |
| `thiserror` | Error type derivation |

---

## License

AGPL-3.0-or-later — the source remains open, as all things should be.