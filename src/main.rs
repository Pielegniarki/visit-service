use app::App;

mod app;
mod db;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let db = db::DB::new("mongodb://localhost:27017").await?;

    App::serve(db).await?;

    Ok(())
}