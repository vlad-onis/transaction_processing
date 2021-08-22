mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // args not available after this line - ownership
    let input = utils::cli_arguments_utils::parse_input(args);

    if let Some(file) = input {
        println!(
            "Input file: {}",
            file.into_os_string().into_string().unwrap()
        );
    }
}
