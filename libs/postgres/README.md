# PostgreSQL

This crate owns database connections, the Core connection adapter, SeaORM schema models in `src/schema/`, and migrations in `src/migration/`. The API uses its library; the optional `cli` feature enables standalone migration commands without starting Loco or Vite.

Run commands from the repository root. Start the local database with:

```sh
docker compose up -d
```

The CLI defaults to `postgres://root:shh@127.0.0.1:7887/compallm`, matching Compose. Override it with `DATABASE_URL` or `--database-url`; `DATABASE_SCHEMA` defaults to `public`.

```sh
export DATABASE_URL='postgres://root:shh@127.0.0.1:7887/compallm'
```

## Migrations

```sh
# Show commands without connecting to the database
cargo run -p compallm-postgres --features cli -- --help

# Show applied and pending migrations
cargo run -p compallm-postgres --features cli -- status

# Generate and register an empty migration in src/migration/; no DB connection
cargo run -p compallm-postgres --features cli -- generate add_organizations

# Apply all pending migrations (also the default when no command is supplied)
cargo run -p compallm-postgres --features cli -- up

# Apply only the next migration
cargo run -p compallm-postgres --features cli -- up -n 1

# Roll back the last migration
cargo run -p compallm-postgres --features cli -- down

# Roll back the last two migrations
cargo run -p compallm-postgres --features cli -- down -n 2
```

Implement both `up` and `down` in generated migrations. Generation writes directly to this crate's migration module regardless of the current working directory. Keep applied migrations unchanged; add another migration for subsequent schema changes. Loco uses the same `Migrator` and currently applies pending migrations on application startup.

These commands remove data or roll back schema changes; use them only when intended:

| Command | Behavior |
| --- | --- |
| `cargo run -p compallm-postgres --features cli -- fresh` | Drop all tables and apply all migrations again. |
| `cargo run -p compallm-postgres --features cli -- refresh` | Roll back all migrations, then apply them again. |
| `cargo run -p compallm-postgres --features cli -- reset` | Roll back all applied migrations. |

The additive `user_sessions` migration records each successful signup/signin and stores the current refresh credential identifier. Its schema is checked in alongside the migration without requiring a live database. `users.last_login_at` remains for compatibility but is no longer written. Applying this migration is a deployment step; existing refresh JWTs without a session ID require signin. No historical sessions are inferred from old timestamps.

## Generate SeaORM entities

After applying the intended migrations, generate database models from the live schema:

```sh
cargo run -p compallm-postgres --features cli -- generate-entity
```

The command always writes to `libs/postgres/src/schema`, regardless of the working directory. It uses the same database URL/schema options as migration commands, skips `seaql_migrations`, and maps timestamps to Chrono types. No separate SeaORM CLI installation is required. It reads the existing database schema without applying migrations.

It rewrites `src/schema/mod.rs` with private model modules and public names such as `compallm_postgres::schema::UserSchema` and `OrganizationSchema`. These names are also re-exported at the crate root. Model names use the singular table name in PascalCase followed by `Schema`. Database enums receive the same suffix: `UserStatus` is exported as `UserStatusSchema`. The original model and enum paths remain private, and the generated SeaORM prelude is not exposed. No manual re-exports are needed; conflicting model or enum alias names cause generation to fail explicitly.

Review generated files before committing. These are persistence models; Core defines its own `User` type and must not import SeaORM models. Regenerating entities replaces files in the schema directory.

## Inspect the database

Open PostgreSQL's SQL shell inside the Compose container:

```sh
docker compose exec postgres psql -U root -d compallm
```

Use `\dt` to list tables, `\d users` to inspect the users table, and `\q` to exit. For database logs:

```sh
docker compose logs --tail 50 postgres
```

## Verify without applying migrations

```sh
cargo check -p compallm-postgres --features cli
cargo test -p compallm-postgres --lib
```

The library tests generate migration SQL without connecting to PostgreSQL. Application request tests may apply migrations automatically.
