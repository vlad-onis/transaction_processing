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
            amount = Some(temp_amount.trim().parse::<f32>().unwrap());
        }

        // Creation of the appropriate transaction.
        // Dummy transaction used to avoid code duplication.
        let transaction = Transaction {
            transaction_type: TransactionType::Default,
            client: String::from(csv_entry.get(1).unwrap())
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
                    return Some(Transaction {
                        transaction_type: TransactionType::Deposit,
                        ..transaction
                    });
                }
                "withdrawal" => {
                    return Some(Transaction {
                        transaction_type: TransactionType::Withdrawal,
                        ..transaction
                    });
                }
                "dispute" => {
                    return Some(Transaction {
                        transaction_type: TransactionType::Dispute,
                        ..transaction
                    });
                }
                "resolve" => {
                    return Some(Transaction {
                        transaction_type: TransactionType::Resolve,
                        ..transaction
                    });
                }
                "chargeback" => {
                    return Some(Transaction {
                        transaction_type: TransactionType::Chargeback,
                        ..transaction
                    });
                }
                _ => {
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
        client: i32,
        available: f32,
        held: f32,
        total: f32,
        locked: bool,
    ) -> Account {
        Account {
            client: client,
            available,
            held,
            total,
            locked,
        }
    }

    pub fn create_default_account(client: i32) -> Account {
        Account {
            client: client,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }
}
