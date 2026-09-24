use std::{str::FromStr, time::Duration};

use sqlx::{ConnectOptions, PgPool, postgres::{PgConnectOptions, PgPoolOptions}};
use tower_sessions::{Expiry, SessionManagerLayer, cookie::time};
use tower_sessions_sqlx_store::PostgresStore;
use tracing::instrument::WithSubscriber;
use tracing_subscriber::{EnvFilter, FmtSubscriber};


pub fn logging(){
    let filter = EnvFilter::builder()
        .with_default_directive(tracing::Level::TRACE.into())
        .from_env_lossy();
    let subscriber = FmtSubscriber::builder()
        // .with_max_level(tracing::Level::TRACE)
        .with_target(false)
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set up logging");

    // tracing::info!("info log");
    // tracing::error!("error log");
    // tracing::debug!("debug log");
    // tracing::trace!("tracing log");
    // tracing::warn!("warn log");
}

pub async fn database_connection() -> PgPool{
    tracing::debug!("Setting up database connection");
    
    let db_url = dotenvy::var("DATABASE_URL").expect("Failed to get database url from .env");

    let options = PgConnectOptions::from_str(&db_url)
        .expect("Failed to parse url")
        .disable_statement_logging();

    let pg_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await
        .expect("Failed to connect to the database");

    tracing::debug!("Successfully connected");

    sqlx::migrate!()
        .run(&pg_pool)
        .await.expect("Failed to migrate");
    tracing::debug!("Successfully migrated");

    pg_pool
}

pub async fn session(pool: PgPool) {
    let session_store = PostgresStore::new(pool);

    session_store.migrate().await.expect("Failed to run session migration");

    let session_layer = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(time::Duration::days(1)));
    

}


