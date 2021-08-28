use crate::model;
use crate::repository;

pub struct TransactionService {
    pub transaction_repository: repository::transaction_repository::TransactionRepository,
    pub account_repository: repository::account_repository::AccountRepository,
}

impl TransactionService {
    pub fn new() -> Option<TransactionService> {
        let transaction_repository =
            repository::transaction_repository::TransactionRepository::new();
        let account_repository = repository::account_repository::AccountRepository::new();

        if (transaction_repository.is_some() && account_repository.is_some()) {
            return Some(TransactionService {
                transaction_repository: transaction_repository.unwrap(),
                account_repository: account_repository.unwrap(),
            });
        }
        None
    }

    pub fn process_transaction(&self, transaction: &model::transaction::Transaction) {
        println!("PROCESSING");
    }
}
