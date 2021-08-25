mod model;
mod repository;
mod utils;

use mongodb::bson::doc;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // args not available after this line - ownership
    let input = utils::cli_arguments_utils::parse_input(args);

    // Todo: to remove
    if let Some(file) = input.as_ref() {
        println!(
            "Input file: {}",
            file.clone().into_os_string().into_string().unwrap()
        );
    } else {
        return;
    }

    let _ = utils::csv_utils::process_file(input.unwrap());

    let transaction_repository = repository::transaction_repository::TransactionRepository::new();
    if let Some(tr) = transaction_repository {
        match tr
            .db_connection
            .collections
            .get(utils::db_utils::TRANSACTION_COLLECTION)
        {
            Some(col) => {
                println!("Inserting: ");
                col.insert_one(doc! {"test": "suta"}, None)
                    .expect("Could not insert");
            }
            _ => println!("Don't have TRANSACTION Collection."),
        }
    }
}
