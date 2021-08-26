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

    pub fn insert_transaction(&self, transaction: &model::transaction::Transaction) -> bool {
        let transaction_document = mongodb::bson::to_document(transaction);

        let transaction_searched = transaction_document.unwrap().clone();
        let transaction_to_be_inserted = transaction_searched.clone();

        let exists = self.db_connection.collections[db_utils::TRANSACTION_COLLECTION]
            .find_one(Some(transaction_searched), None)
            .unwrap()
            .is_some();
        if exists {
            return false;
        }

        self.db_connection.collections[db_utils::TRANSACTION_COLLECTION]
            .insert_one(transaction_to_be_inserted, None)
            .expect("Could not insert");

        true
    }

    pub fn update_transaction(
        &self,
        old_transaction: &model::transaction::Transaction,
        new_transaction: &model::transaction::Transaction,
    ) {
        let old_transaction_document = mongodb::bson::to_document(old_transaction);
        let new_transaction_document = mongodb::bson::to_document(new_transaction);

        self.db_connection.collections[db_utils::TRANSACTION_COLLECTION].find_one_and_replace(
            old_transaction_document.unwrap(),
            new_transaction_document.unwrap(),
            None,
        );
    }
}
