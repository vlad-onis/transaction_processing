use std::io;
use mongodb::{
    sync,
    bson::doc,

};

const ACCOUNT_COLLECTION: &str = "Account";
const TRANSACTION_COLLECTION: &str = "Transaction";

pub struct DatabaseAccess {
    pub client: sync::Client,
    pub database: sync::Database,
    pub collections: Vec<sync::Collection>
}

impl DatabaseAccess {
    fn setup_db() -> Result<DatabaseAccess, mongodb::error::Error>{

        let client = sync::Client::with_uri_str("mongodb+srv://tester:transactiontest123@cluster0.xj3t7.mongodb.net/myFirstDatabase?retryWrites=true&w=majority")?;
        let database = client.database("transaction_processing_db");
        let collections = vec!(
            database.collection(ACCOUNT_COLLECTION),
            database.collection(TRANSACTION_COLLECTION));

        Ok(DatabaseAccess {
            client,
            database,
            collections
        })
    }

    pub fn new() -> Result<DatabaseAccess, mongodb::error::Error> {
        DatabaseAccess::setup_db()
    }
}