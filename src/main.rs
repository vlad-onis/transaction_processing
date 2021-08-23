mod model;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // args not available after this line - ownership
    let input = utils::cli_arguments_utils::parse_input(args);

    if let Some(file) = input.as_ref() {
        println!(
            "Input file: {}",
            file.clone().into_os_string().into_string().unwrap()
        );
    } else {
        return;
    }

    let _ = utils::csv_utils::process_file(input.unwrap());
}
