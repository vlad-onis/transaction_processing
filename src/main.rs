mod model;
mod utils;
mod repository;

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

    let db = utils::db_utils::DatabaseAccess::new();

    for coll in db.unwrap().collections {
        coll.insert_one(doc!{"test": 13}, None);
    }
}
