use crate::model::transaction::Transaction;
use crate::model::transaction::TransactionType;

use csv::StringRecord;

pub struct TransactionFactory {}

impl TransactionFactory {
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
                .parse::<u16>()
                .unwrap(),
            transaction_id: String::from(csv_entry.get(2).unwrap())
                .trim()
                .parse::<u32>()
                .unwrap(),
            amount,
        };

        if let Some(tx_type) = csv_entry.get(0) {
            match tx_type {
                "deposit" => {
                    println!("IT'S A DEPOSIT"); // Todo: remove
                    return Some(Transaction {
                        transaction_type: TransactionType::Deposit,
                        ..transaction
                    });
                }
                "withdrawal" => {
                    println!("IT'S A WITHDRAW"); // Todo: remove
                    return Some(Transaction {
                        transaction_type: TransactionType::Withdrawal,
                        ..transaction
                    });
                }
                "dispute" => {
                    println!("IT'S A DISPUTE"); // Todo: remove
                    return Some(Transaction {
                        transaction_type: TransactionType::Dispute,
                        ..transaction
                    });
                }
                "resolve" => {
                    println!("IT'S A RESOLVE"); // Todo: remove
                    return Some(Transaction {
                        transaction_type: TransactionType::Resolve,
                        ..transaction
                    });
                }
                "chargeback" => {
                    println!("IT'S A CHARGEBACK"); // Todo: remove
                    return Some(Transaction {
                        transaction_type: TransactionType::Chargeback,
                        ..transaction
                    });
                }
                _ => {
                    println!("Transaction Factory: Transaction type can't be handled yet");
                    return None;
                }
            }
        }

        None
    }
}
