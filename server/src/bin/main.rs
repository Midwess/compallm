use cosin_postgres::Migrator;
use cosin_server::app::App;
use loco_rs::cli;

#[tokio::main]
async fn main() -> Result<(), Box<loco_rs::Error>> {
    cli::main::<App, Migrator>().await.map_err(Box::new)
}
