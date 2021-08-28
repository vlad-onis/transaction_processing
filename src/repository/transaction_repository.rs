use crate::model;
use crate::utils::db_utils;

use mongodb::bson::doc;

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

    // Todo: Uncomment when the following will be used. Delete otherwise
    // pub fn delete_transaction(&self, transaction: &model::transaction::Transaction) {
    //     let transaction_document = mongodb::bson::to_document(transaction);
    //     self.db_connection.collections[db_utils::TRANSACTION_COLLECTION].find_one_and_delete(
    //         transaction_document.unwrap(),
    //         None,
    //     ).expect("Could not delete transaction");
    // }
    //
    // pub fn update_transaction(
    //     &self,
    //     old_transaction: &model::transaction::Transaction,
    //     new_transaction: &model::transaction::Transaction,
    // ) {
    //     let old_transaction_document = mongodb::bson::to_document(old_transaction);
    //     let new_transaction_document = mongodb::bson::to_document(new_transaction);
    //
    //     self.db_connection.collections[db_utils::TRANSACTION_COLLECTION].find_one_and_replace(
    //         old_transaction_document.unwrap(),
    //         new_transaction_document.unwrap(),
    //         None,
    //     ).expect("Could not update transaction");
    // }
    //
    // pub fn find_transaction_by_id(&self, transaction_id: i32) -> bool {
    //
    //     let transaction_id_document = doc! {
    //         "transaction_id": transaction_id
    //     };
    //
    //     let found_transaction = self.db_connection.collections[db_utils::TRANSACTION_COLLECTION].find_one(transaction_id_document, None);
    //     found_transaction.unwrap().is_some()
    // }
}
