use sea_orm::{Database, DatabaseConnection, DbErr};
use tracing::info;

pub async fn establish_connection(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    info!("Connecting to database...");
    let db = Database::connect(database_url).await?;
    info!("Database connection established");
    Ok(db)
}
