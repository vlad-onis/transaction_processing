use crate::service;
use crate::utils::factory;

use csv::StringRecord;
use std::error::Error;
use std::io;
use std::path;

use crate::model::transaction::{Transaction, TRANSACTION_FIELDS};

fn create_output(
    transaction_service: service::transaction_service::TransactionService,
) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_writer(io::stdout());

    let accounts = transaction_service.get_accounts();
    if accounts.is_some() {
        let accounts = accounts.unwrap();
        for account in accounts {
            writer.serialize(account)?;
            writer.flush()?;
        }
    }
    Ok(())
}

/// Creates a service responsible for transaction handling.
/// It creates transactions by reading the csv file line by line. One line representing a valid transaction.
/// CAUTION: If an entry is invalid in any way, the whole program stops at that transaction.
/// # Arguments
/// * input-path - Path object representing the path on the local filesystem to the csv file.
pub fn process_file(input_path: path::PathBuf) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(input_path)?;

    let transaction_service = service::transaction_service::TransactionService::new().unwrap();

    for (_entry_count, item) in reader.records().enumerate() {
        let record = item?;

        let transaction = process_entry(&record);
        if transaction.is_none() {
            // println!("Transaction {} could not be parsed", entry_count);
            continue;
        }

        let mut transaction = transaction.unwrap();
        transaction_service.process_transaction(&mut transaction)
    }

    create_output(transaction_service)
}

/// Processes a single csv entry, returns a Transaction object on success, None otherwise.
fn process_entry(entry: &StringRecord) -> Option<Transaction> {
    if entry.len() != TRANSACTION_FIELDS {
        // println!("Entry {:?} is not valid", entry);
        return None;
    }

    factory::TransactionFactory::create_transaction(entry)
}
