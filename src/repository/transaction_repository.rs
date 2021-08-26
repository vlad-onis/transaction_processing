use crate::model;
use crate::utils::db_utils;

pub struct TransactionRepository {
    pub db_connection: db_utils::DatabaseAccess,
}

impl TransactionRepository {
    pub fn new() -> Option<TransactionRepository> {
        let database_access = db_utils::DatabaseAccess::new();

        if let Ok(db_access) = database_access {
            return Some(TransactionRepository {
                db_connection: db_access,
            });
        }
        None
    }

    pub fn insert_transaction(&self, transaction: &model::transaction::Transaction) {
        let transaction_document = mongodb::bson::to_document(transaction);

        self.db_connection.collections[db_utils::TRANSACTION_COLLECTION]
            .insert_one(transaction_document.unwrap(), None)
            .expect("Could not insert");
    }
}
