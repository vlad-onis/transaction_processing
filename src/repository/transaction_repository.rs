use crate::model::transaction::Transaction;
use crate::utils::db_utils::*;

pub struct TransactionRepository {
    pub db_connection: DatabaseAccess,
}

impl TransactionRepository {
    pub fn new() -> Option<TransactionRepository> {
        let database_access = DatabaseAccess::new();

        if let Ok(db_access) = database_access {
            return Some(TransactionRepository {
                db_connection: db_access,
            });
        }
        None
    }

    pub fn insert_transaction(transaction: &Transaction) {
        println!("Inserting transaction {:?}", transaction);
    }
}
