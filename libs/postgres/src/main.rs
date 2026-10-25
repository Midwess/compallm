use clap::{Parser, Subcommand};
use compallm_postgres::{Migrator, codegen::SchemaAliases};
use sea_orm::{ConnectOptions, Database};
use sea_orm_cli::{MigrateSubcommands, run_migrate_generate};
use std::error::Error;

#[derive(Parser)]
#[command(
    version,
    about = "Manage Compallm PostgreSQL migrations and schema entities"
)]
struct PostgresCli {
    #[arg(
        short = 'u',
        long,
        env = "DATABASE_URL",
        default_value = "postgres://root:shh@127.0.0.1:7887/compallm",
        hide_env_values = true,
        hide_default_value = true,
        global = true
    )]
    database_url: String,

    #[arg(
        short = 's',
        long,
        env = "DATABASE_SCHEMA",
        default_value = "public",
        global = true
    )]
    database_schema: String,

    #[arg(short = 'v', long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<PostgresCommand>,
}

#[derive(Subcommand)]
enum PostgresCommand {
    /// Generate SeaORM entities from the database into this crate's src/schema.
    #[command(alias = "entity")]
    GenerateEntity,
    #[command(flatten)]
    Migration(MigrateSubcommands),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = PostgresCli::parse();
    let command = match cli.command {
        Some(PostgresCommand::GenerateEntity) => {
            // Parse SeaORM's defaults instead of duplicating its codegen options.
            let generator = sea_orm_cli::Cli::try_parse_from([
                "sea-orm-cli",
                "generate",
                "entity",
                "--database-url",
                &cli.database_url,
                "--database-schema",
                &cli.database_schema,
                "--output-dir",
                concat!(env!("CARGO_MANIFEST_DIR"), "/src/schema"),
                "--ignore-tables",
                "seaql_migrations",
                "--date-time-crate",
                "chrono",
            ])?;
            if let sea_orm_cli::Commands::Generate { command } = generator.command {
                sea_orm_cli::run_generate_command(command, cli.verbose).await?;
                SchemaAliases::generate(std::path::Path::new(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/src/schema"
                )))?;
            }
            return Ok(());
        }
        Some(PostgresCommand::Migration(command)) => Some(command),
        None => None,
    };
    match command {
        Some(MigrateSubcommands::Generate {
            migration_name,
            local_time,
            ..
        }) => {
            // SeaORM's default CLI generates relative to the caller's directory.
            // Target our existing migration module explicitly instead.
            run_migrate_generate(
                concat!(env!("CARGO_MANIFEST_DIR"), "/src/migration"),
                &migration_name,
                !local_time,
            )?;
        }
        Some(MigrateSubcommands::Init) => {
            return Err("the migration module is already initialized; use generate <NAME>".into());
        }
        command => {
            let mut options = ConnectOptions::new(cli.database_url);
            options
                .set_schema_search_path(cli.database_schema)
                .max_connections(1);
            let db = Database::connect(options).await?;
            sea_orm_migration::cli::run_migrate(Migrator, &db, command, cli.verbose).await?;
            db.close().await?;
        }
    }
    Ok(())
}
