use crate::service;
use crate::utils::errors::ServiceCreationError;
use crate::utils::factory;

use csv::StringRecord;
use std::error::Error;
use std::path;

use crate::model::transaction::{Transaction, TRANSACTION_FIELDS};

/// Creates a service responsible for transaction handling.
/// It creates transactions by reading the csv file line by line. One line representing a valid transaction.
/// CAUTION: If an entry is invalid in any way, the whole program stops at that transaction.
/// # Arguments
/// * input-path - Path object representing the path on the local filesystem to the csv file.
pub fn process_file(input_path: path::PathBuf) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(input_path)?;

    let transaction_service = service::transaction_service::TransactionService::new();

    if transaction_service.is_none() {
        println!("Could not create service, stopping...");
        return Err(Box::new(ServiceCreationError(
            "Could not create transaction service".into(),
        )));
    }

    let transaction_service = transaction_service.unwrap();

    for (entry_count, item) in reader.records().enumerate() {
        let record = item?;

        let transaction = process_entry(&record);
        if transaction.is_none() {
            println!("Transaction {} could not be parsed", entry_count);
            continue;
        }

        let mut transaction = transaction.unwrap();
        transaction_service.process_transaction(&mut transaction)
    }

    Ok(())
}

/// Processes a single csv entry, returns a Transaction object on success, None otherwise.
fn process_entry(entry: &StringRecord) -> Option<Transaction> {
    if entry.len() != TRANSACTION_FIELDS {
        println!("Entry {:?} is not valid", entry);
        return None;
    }

    factory::TransactionFactory::create_transaction(entry)
}
