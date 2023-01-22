use app::App;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod db;
mod http_client;
mod models;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
        
    let db = db::DB::new("mongodb://localhost:27017").await?;

    let client = http_client::HttpClient::new();
    
    App::serve(db, client).await?;

    Ok(())
}