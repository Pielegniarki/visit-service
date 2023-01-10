use mongodb::{Client, Collection, Database, bson::doc};

pub mod schemas;

pub struct DB {
    db: Database
}

impl DB {
    pub async fn new(uri: &str) -> Result<DB, mongodb::error::Error> {
        let client = Client::with_uri_str(uri).await?;

        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await?;
        println!("Connected successfully.");

        let db = client.database("pielegniarki");

        Ok(DB {db})
    }

    pub fn collections(&self) -> CollectionSelector {
        CollectionSelector { db: &self.db }
    }
}

pub struct CollectionSelector<'a> {
    db: &'a Database
}

impl CollectionSelector<'_> {
    pub fn visit(&self) -> Collection<schemas::Visit> {
        self.db.collection("visit")
    }
}