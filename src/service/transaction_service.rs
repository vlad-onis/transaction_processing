use crate::model;
use crate::model::transaction::TransactionType;
use crate::repository;
use crate::utils::errors;
use crate::utils::errors::TransactionFailedError;
use crate::utils::factory;

pub struct TransactionService {
    pub transaction_repository: repository::transaction_repository::TransactionRepository,
    pub account_repository: repository::account_repository::AccountRepository,
}

impl TransactionService {
    pub fn new() -> Option<TransactionService> {
        let transaction_repository =
            repository::transaction_repository::TransactionRepository::new();
        let account_repository = repository::account_repository::AccountRepository::new();

        if transaction_repository.is_some() && account_repository.is_some() {
            return Some(TransactionService {
                transaction_repository: transaction_repository.unwrap(),
                account_repository: account_repository.unwrap(),
            });
        }
        None
    }

    pub fn process_transaction(&self, transaction: &mut model::transaction::Transaction) {
        let existent_account = self
            .account_repository
            .find_account_by_client_id(transaction.client_id);

        let mut account = factory::AccountFactory::create_default_account(transaction.client_id);
        if existent_account.is_none() {
            self.account_repository.insert_account(&account);
            if transaction.transaction_type != model::transaction::TransactionType::Deposit {
                println!(
                    "Transaction with transaction_id={} cannot be performed because the account_id={} is new, only DEPOSIT is allowed",
                     transaction.transaction_id,
                     account.client_id);
                return; // Todo: return type???
            }
        } else {
            let existent_account = existent_account.unwrap();
            account = factory::AccountFactory::create_account(
                existent_account.client_id,
                existent_account.available,
                existent_account.total,
                existent_account.held,
                existent_account.locked,
            );
        }

        if account.locked {
            println!(
                "Transaction id={} cannot be processed at this time because the account client_id={} is locked",
                 transaction.transaction_id,
                 transaction.client_id);
            return; // Todo: return type???
        }

        match transaction.transaction_type {
            TransactionType::Deposit => {
                if self.process_deposit(&mut account, &transaction).is_ok() {
                    self.transaction_repository.insert_transaction(transaction);
                } else {
                    return;
                }
            }
            TransactionType::Withdrawal => {
                if self.process_withdrawal(&mut account, &transaction).is_ok() {
                    self.transaction_repository.insert_transaction(transaction);
                } else {
                    return;
                }
            }

            TransactionType::Dispute => {
                let disputed_transaction = self
                    .transaction_repository
                    .find_transaction_by_id(transaction.transaction_id);
                if disputed_transaction.is_none() {
                    println!(
                        "Disputed transaction with transaction_id={} does not exist",
                        transaction.transaction_id
                    );
                    return;
                }
                let mut disputed_transaction = disputed_transaction.unwrap();
                if self
                    .process_dispute(&mut account, &disputed_transaction)
                    .is_err()
                {
                    return;
                }
                disputed_transaction.disputed = true;
                self.transaction_repository
                    .update_transaction(disputed_transaction.transaction_id, &disputed_transaction);
            }
            TransactionType::Resolve => {
                let disputed_transaction = self
                    .transaction_repository
                    .find_transaction_by_id(transaction.transaction_id);
                if disputed_transaction.is_none() {
                    println!(
                        "Disputed transaction with transaction_id={} does not exist",
                        transaction.transaction_id
                    );
                    return;
                }
                let mut disputed_transaction = disputed_transaction.unwrap();
                if !disputed_transaction.disputed {
                    println!("Transaction is not under dispute, RESOLVE will not be performed");
                    return;
                }

                if self
                    .process_resolve(&mut account, &disputed_transaction)
                    .is_err()
                {
                    return;
                }
                disputed_transaction.disputed = false;
                self.transaction_repository
                    .update_transaction(disputed_transaction.transaction_id, &disputed_transaction);
            }
            TransactionType::Chargeback => {
                let disputed_transaction = self
                    .transaction_repository
                    .find_transaction_by_id(transaction.transaction_id);
                if disputed_transaction.is_none() {
                    println!(
                        "Disputed transaction with transaction_id={} does not exist",
                        transaction.transaction_id
                    );
                    return;
                }
                let mut disputed_transaction = disputed_transaction.unwrap();
                if !disputed_transaction.disputed {
                    println!("Transaction is not under dispute, RESOLVE will not be performed");
                    return;
                }

                if self
                    .process_chargeback(&mut account, &disputed_transaction)
                    .is_err()
                {
                    return;
                }
                disputed_transaction.disputed = false;
                self.transaction_repository
                    .update_transaction(disputed_transaction.transaction_id, &disputed_transaction);
            }
            TransactionType::Default => {
                println!("Not yet implemented")
            }
        }

        if self
            .account_repository
            .update_account(account.client_id, &account)
        {
            println!("Account with client_id={} was updated", account.client_id);
        }
    }

    fn process_withdrawal(
        &self,
        account: &mut model::account::Account,
        transaction: &model::transaction::Transaction,
    ) -> Result<(), TransactionFailedError> {
        if transaction.amount.is_some() {
            let amount = transaction.amount.unwrap();
            return if account.total >= amount && account.available >= amount {
                account.total -= amount;
                account.available -= amount;
                Ok(())
            } else {
                println!(
                    "Insuficient funds for WITHDRAWAL with transaction_id={}",
                    transaction.transaction_id
                );
                Err(TransactionFailedError(String::from("Withdrawal failed")))
            };
        } else {
            println!(
                "Amount not available: WITHDRAWAL did not modify account with client_id={}",
                account.client_id
            );
            Err(TransactionFailedError(String::from("Deposit failed")))
        }
    }

    fn process_deposit(
        &self,
        account: &mut model::account::Account,
        transaction: &model::transaction::Transaction,
    ) -> Result<(), errors::TransactionFailedError> {
        if transaction.amount.is_some() {
            let amount = transaction.amount.unwrap();
            account.total += amount;
            account.available += amount;
            return Ok(());
        } else {
            println!(
                "Amount not available: DEPOSIT did not modify account with client_id={}",
                account.client_id
            );
        }

        Err(TransactionFailedError(String::from("Deposit failed")))
    }

    fn process_dispute(
        &self,
        account: &mut model::account::Account,
        transaction: &model::transaction::Transaction,
    ) -> Result<(), TransactionFailedError> {
        if transaction.amount.is_some() {
            let amount = transaction.amount.unwrap();
            account.available -= amount;
            account.held += amount;
            return Ok(());
        } else {
            println!(
                "Amount not available: DISPUTE did not modify account with client_id={}",
                account.client_id
            );
        }

        Err(TransactionFailedError(String::from("Deposit failed")))
    }

    fn process_resolve(
        &self,
        account: &mut model::account::Account,
        transaction: &model::transaction::Transaction,
    ) -> Result<(), TransactionFailedError> {
        if transaction.amount.is_some() {
            let amount = transaction.amount.unwrap();
            account.available += amount;
            account.held -= amount;
            return Ok(());
        } else {
            println!(
                "Amount not available: RESOLVE did not modify account with client_id={}",
                account.client_id
            );
        }

        Err(TransactionFailedError(String::from("Deposit failed")))
    }

    fn process_chargeback(
        &self,
        account: &mut model::account::Account,
        transaction: &model::transaction::Transaction,
    ) -> Result<(), TransactionFailedError> {
        if transaction.amount.is_some() {
            let amount = transaction.amount.unwrap();
            account.total -= amount;
            account.held -= amount;
            account.locked = true;
            return Ok(());
        } else {
            println!(
                "Amount not available: CHARGEBACK did not modify account with client_id={}",
                account.client_id
            );
        }

        Err(TransactionFailedError(String::from("Deposit failed")))
    }
}
