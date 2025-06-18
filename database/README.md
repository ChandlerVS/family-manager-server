# Database Migration System

This module provides a robust database migration system for managing schema changes in PostgreSQL.

## Features

- **Automatic Migration Tracking**: Migrations are tracked in a `migrations` table to ensure they're only applied once
- **Rollback Support**: Migrations can be rolled back individually
- **Status Checking**: Check which migrations have been applied
- **Safe Execution**: Migrations are executed within transactions for safety

## Usage

### Running Migrations

```rust
use database::{run_database_migrations, get_migration_status};

// Run all pending migrations
run_database_migrations("postgresql://user:password@localhost/dbname").await?;

// Check migration status
let status = get_migration_status("postgresql://user:password@localhost/dbname").await?;
for migration in status {
    println!("Migration {}: {}", migration.name, if migration.applied { "Applied" } else { "Pending" });
}
```

### Using DatabaseManager

```rust
use database::{initialize_database, get_database_manager};

// Initialize database and run migrations
initialize_database("postgresql://user:password@localhost/dbname").await?;

// Get the database manager
if let Some(manager) = get_database_manager() {
    // Run migrations
    manager.run_migrations().await?;
}
```

### Creating New Migrations

1. Create a new migration file in `src/migrations/` (e.g., `m002_create_families_table.rs`)
2. Implement the `Migration` trait:

```rust
use std::future::Future;
use std::pin::Pin;
use crate::migrations::Migration;

pub struct CreateFamiliesTableMigration {}

impl Migration for CreateFamiliesTableMigration {
    fn name(&self) -> &str {
        "m002_create_families_table"
    }

    fn up(&self, pool: &sqlx::PgPool) -> Pin<Box<dyn Future<Output = Result<(), crate::error::DatabaseError>> + Send>> {
        let pool = pool.clone();
        Box::pin(async move {
            let sql = r#"
                CREATE TABLE IF NOT EXISTS families (
                    id SERIAL PRIMARY KEY,
                    name VARCHAR(255) NOT NULL,
                    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
                );
            "#;
            sqlx::query(sql).execute(&pool).await?;
            Ok(())
        })
    }

    fn down(&self, pool: &sqlx::PgPool) -> Pin<Box<dyn Future<Output = Result<(), crate::error::DatabaseError>> + Send>> {
        let pool = pool.clone();
        Box::pin(async move {
            sqlx::query("DROP TABLE IF EXISTS families").execute(&pool).await?;
            Ok(())
        })
    }
}
```

3. Add the migration to `get_available_migrations()` in `src/migrations/mod.rs`:

```rust
fn get_available_migrations(&self) -> Vec<Box<dyn Migration>> {
    vec![
        Box::new(m001_create_users_table::CreateUsersTableMigration {}),
        Box::new(m002_create_families_table::CreateFamiliesTableMigration {}),
    ]
}
```

## Migration Table Schema

The system uses a `migrations` table to track applied migrations:

```sql
CREATE TABLE migrations (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    up_sql TEXT NOT NULL,
    down_sql TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    applied_at TIMESTAMP WITH TIME ZONE
);
```

## Error Handling

The migration system uses the `DatabaseError` enum for error handling:

- `ConnectionError`: Database connection issues
- `MigrationError`: Migration-specific errors (e.g., unknown migration)

## Safety Features

- **Idempotent**: Migrations can be run multiple times safely
- **Transaction-based**: Each migration runs in its own transaction
- **Rollback Support**: Migrations can be rolled back if needed
- **Status Tracking**: Clear visibility into which migrations have been applied 