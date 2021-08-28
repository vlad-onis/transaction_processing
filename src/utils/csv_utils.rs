use crate::service;
use crate::utils::factory;

use csv::StringRecord;
use std::error::Error;
use std::fmt;
use std::path;

use crate::model::transaction::{Transaction, TRANSACTION_FIELDS};

#[derive(Debug)]
struct ServiceCreationError(String);

impl fmt::Display for ServiceCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}
impl Error for ServiceCreationError {}

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

        let transaction = transaction.unwrap();
        transaction_service.process_transaction(&transaction)
    }

    Ok(())
}

fn process_entry(entry: &StringRecord) -> Option<Transaction> {
    if entry.len() != TRANSACTION_FIELDS {
        println!("Entry {:?} is not valid", entry);
        return None;
    }

    factory::TransactionFactory::create_transaction(entry)
}
