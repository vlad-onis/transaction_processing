use crate::model::account::Account;
use crate::model::transaction::Transaction;
use crate::model::transaction::TransactionType;

use csv::StringRecord;

pub struct TransactionFactory {}

impl TransactionFactory {
    /// Creates a transaction object based on the input from the csv file
    /// # Arguments
    /// * csv_entry - StringRecord representing a single valid entry in the csv file
    pub fn create_transaction(csv_entry: &StringRecord) -> Option<Transaction> {
        let mut amount: Option<f32> = None;
        let temp_amount = String::from(csv_entry.get(3).unwrap());

        if !temp_amount.is_empty() {
            // Application could panic here
            amount = Some(temp_amount.trim().parse::<f32>().unwrap());
        }

        // Creation of the appropriate transaction.
        // Dummy transaction used to avoid code duplication.
        let transaction = Transaction {
            transaction_type: TransactionType::Default,
            client_id: String::from(csv_entry.get(1).unwrap())
                .trim()
                .parse::<i32>()
                .unwrap(),
            transaction_id: String::from(csv_entry.get(2).unwrap())
                .trim()
                .parse::<i32>()
                .unwrap(),
            amount,
            disputed: false,
        };

        if let Some(tx_type) = csv_entry.get(0) {
            match tx_type.trim() {
                "deposit" => {
                    // println!("Transaction type: DEPOSIT"); // Todo: remove
                    return Some(Transaction {
                        transaction_type: TransactionType::Deposit,
                        ..transaction
                    });
                }
                "withdrawal" => {
                    // println!("Transaction type: WITHDRAWAL"); // Todo: remove
                    return Some(Transaction {
                        transaction_type: TransactionType::Withdrawal,
                        ..transaction
                    });
                }
                "dispute" => {
                    // println!("Transaction type: DISPUTE"); // Todo: remove
                    return Some(Transaction {
                        transaction_type: TransactionType::Dispute,
                        ..transaction
                    });
                }
                "resolve" => {
                    // println!("Transaction type: RESOLVE"); // Todo: remove
                    return Some(Transaction {
                        transaction_type: TransactionType::Resolve,
                        ..transaction
                    });
                }
                "chargeback" => {
                    // println!("Transaction type: CHARGEBACK"); // Todo: remove
                    return Some(Transaction {
                        transaction_type: TransactionType::Chargeback,
                        ..transaction
                    });
                }
                _ => {
                    // println!("Transaction Factory: Transaction type can't be handled yet");
                    return None;
                }
            }
        }

        None
    }
}

pub struct AccountFactory {}

impl AccountFactory {
    pub fn create_account(
        client_id: i32,
        available: f32,
        held: f32,
        total: f32,
        locked: bool,
    ) -> Account {
        Account {
            client_id,
            available,
            held,
            total,
            locked,
        }
    }

    pub fn create_default_account(client_id: i32) -> Account {
        Account {
            client_id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }
}
