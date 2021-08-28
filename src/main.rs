mod model;
mod repository;
mod service;
mod test;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // args not available after this line - ownership
    let input_path = utils::cli_arguments_utils::parse_input(args);

    // Todo: to remove
    if let Some(file) = input_path.as_ref() {
        println!(
            "Input file: {}",
            file.clone().into_os_string().into_string().unwrap()
        );
    } else {
        return;
    }

    let res = utils::csv_utils::process_file(input_path.unwrap());
    println!("{:?}", res);
}
