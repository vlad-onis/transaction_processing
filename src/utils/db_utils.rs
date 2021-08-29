use mongodb::sync;
use std::collections::HashMap;

pub const ACCOUNT_COLLECTION: &str = "Account";
pub const TRANSACTION_COLLECTION: &str = "Transaction";
const COLLECTION_NAMES: &[&str] = &[ACCOUNT_COLLECTION, TRANSACTION_COLLECTION];

pub struct DatabaseAccess {
    pub client: sync::Client,
    pub database: sync::Database,
    pub collections: HashMap<String, sync::Collection>,
}

impl DatabaseAccess {
    /// Sets up the connection to the database. Returns the connection on success, Err otherwise.
    fn setup_db() -> Result<DatabaseAccess, mongodb::error::Error> {
        let client = sync::Client::with_uri_str("mongodb+srv://tester:transactiontest123@cluster0.xj3t7.mongodb.net/myFirstDatabase?retryWrites=true&w=majority")?;
        let database = client.database("transaction_processing_db");

        let mut collections: HashMap<String, sync::Collection> = HashMap::new();
        for col in COLLECTION_NAMES {
            let collection_name = col.to_string();
            collections.insert(collection_name, database.collection(col));
        }

        Ok(DatabaseAccess {
            client,
            database,
            collections,
        })
    }

    /// Creates a new DatabaseAccess object using the setup function.
    pub fn new() -> Result<DatabaseAccess, mongodb::error::Error> {
        DatabaseAccess::setup_db()
    }
}
