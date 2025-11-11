use dotenv::dotenv;
use std::sync::Arc;
use template_rust_backend::{config, routes};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter("template_rust_backend=debug,tower_http=debug,sqlx=debug")
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .init();

    let db_config = config::DatabaseConfig::from_env()
        .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    let db = Arc::new(db_config.connect().await?);

    let config = Arc::new(
        config::Config::from_env()
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))?,
    );

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "run_migrations" => {
                tracing::info!("Running migrations...");
                run_migrations(&db).await?;
                return Ok(());
            }
            _ => {}
        }
    }

    let app = routes::create_router(db.clone(), config.clone());

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.server_host, config.server_port))
            .await?;
    tracing::info!(
        "Server listening on http://{}:{}",
        config.server_host,
        config.server_port
    );
    axum::serve(listener, app).await?;

    Ok(())
}
async fn run_migrations(db: &sea_orm::DatabaseConnection) -> anyhow::Result<()> {
    use migration::Migrator;
    use sea_orm_migration::prelude::*;

    tracing::info!("Starting migrations...");
    let result = Migrator::up(db, None).await;
    if let Err(e) = &result {
        tracing::error!("Migration error: {:?}", e);
        tracing::error!("Full error: {:#?}", e);
    }
    result?;
    tracing::info!("Migrations completed successfully");
    Ok(())
}
