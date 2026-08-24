use forge::{
    config::AppConfig,
    database::connect_db,
    modules::{
        access_control::seeder::AccessControlSeeder,
        users::seeder::UserSeeder,
    },
    shared::logger,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::load()?;
    let _guard = logger::init_tracing("info");

    tracing::info!("Connecting to database for seeding...");
    let db = connect_db(&app_config.infra.db).await?;

    tracing::info!("1. Seeding Access Control module (roles, permissions, role-permissions)...");
    AccessControlSeeder::seed_all(&db).await?;

    tracing::info!("2. Seeding Users module (users, profiles, user-roles)...");
    UserSeeder::seed_all(&db).await?;

    tracing::info!("✅ All database seeders completed successfully!");
    Ok(())
}
