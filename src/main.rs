mod model;
mod repository;
mod service;
mod test;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // args not available after this line - ownership
    let input_path = utils::cli_arguments_utils::parse_input(args).unwrap();

    let res = utils::csv_utils::process_file(input_path);
    if res.is_err() {
        // println!("The file was not processed entirely due to an error in the input file");
    }
}
