use crate::model;
use crate::utils::db_utils;

use mongodb::bson::doc;
use std::error;

pub struct TransactionRepository {
    pub db_connection: db_utils::DatabaseAccess,
}

impl TransactionRepository {
    /// Returns a TransactionRepository if the database connection can be established, None otherwise
    pub fn new() -> Result<TransactionRepository, Box<dyn error::Error>> {
        let database_access = db_utils::DatabaseAccess::new();

        match database_access {
            Ok(db_access) => {
                db_access.collections[db_utils::TRANSACTION_COLLECTION]
                    .drop(None)
                    .expect("Could not drop transaction collection");
                return Ok(TransactionRepository {
                    db_connection: db_access,
                });
            }
            Err(error) => {
                Err(error.into())
            }
        }
    }

    /// Inserts the given transaction in the Transaction collection of the database.
    /// Returns true on success, false otherwise.
    /// # Arguments
    ///
    /// * transaction - a Transaction to be inserted into the db
    pub fn insert_transaction(&self, transaction: &model::transaction::Transaction) -> bool {
        let transaction_searched = doc! {
            "transaction_id" : transaction.transaction_id
        };

        let transaction_exists = self.db_connection.collections[db_utils::TRANSACTION_COLLECTION]
            .find_one(Some(transaction_searched), None)
            .unwrap()
            .is_some();
        if transaction_exists {
            return false;
        }
        let transaction_document = mongodb::bson::to_document(transaction).unwrap();
        self.db_connection.collections[db_utils::TRANSACTION_COLLECTION]
            .insert_one(transaction_document, None)
            .expect("Could not insert");

        true
    }

    /// Updates the transaction represented by an ID with a new given value
    /// Returns true on success, false otherwise.
    /// # Arguments
    /// * old_transaction_id - i32 representing the transaction to be updated
    /// * new_transaction - Transaction holding the new values for update.
    pub fn update_transaction(
        &self,
        old_transaction_id: i32,
        new_transaction: &model::transaction::Transaction,
    ) {
        let old_transaction_document = doc! {
            "transaction_id" : old_transaction_id
        };

        let new_transaction_document = mongodb::bson::to_document(new_transaction);

        self.db_connection.collections[db_utils::TRANSACTION_COLLECTION]
            .find_one_and_replace(
                old_transaction_document,
                new_transaction_document.unwrap(),
                None,
            )
            .expect("Could not update transaction");
    }

    /// Searches for a transaction by it's transaction id.
    /// Returns Option<Transaction>.
    /// # Arguments
    /// * transaction_id - i32
    pub fn find_transaction_by_id(
        &self,
        transaction_id: i32,
    ) -> Option<model::transaction::Transaction> {
        let transaction_searched = doc! {
            "transaction_id": transaction_id
        };

        let transaction_result = self.db_connection.collections[db_utils::TRANSACTION_COLLECTION]
            .find_one(transaction_searched, None);

        // TODO: Clippy improvement
        if transaction_result.is_ok() {
            let transaction_document = transaction_result.unwrap();

            // TODO: Clippy improvement
            if transaction_document.is_none() {
                return None;
            }

            let transaction_document = transaction_document.unwrap();
            let transaction = mongodb::bson::from_document::<model::transaction::Transaction>(
                transaction_document,
            );
            // TODO: clippy improvement
            if transaction.is_ok() {
                return Some(transaction.unwrap());
            }
        }

        None
    }
}
