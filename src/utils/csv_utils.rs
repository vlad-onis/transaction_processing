use super::factory::TransactionFactory;

use csv::StringRecord;
use std::error::Error;
use std::path;

use crate::model::transaction::{Transaction, TRANSACTION_FIELDS};

pub fn process_file(input_path: path::PathBuf) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(input_path)?;

    for (entry_count, item) in reader.records().enumerate() {
        let record = item?;

        let transaction = process_entry(&record);

        if transaction.is_none() {
            println!("Transaction {} could not be parsed", entry_count);
        }
    }

    Ok(())
}

fn process_entry(entry: &StringRecord) -> Option<Transaction> {
    if entry.len() != TRANSACTION_FIELDS {
        println!("Entry {:?} is not valid", entry);
        return None;
    }

    TransactionFactory::create_transaction(entry)
}