use std::ffi::OsStr;
use std::path;

/// Returns a path to the csv file if there is a valid csv file, None otherwise.
/// # Arguments
/// * args - Vector of command line arguments.
pub fn parse_input(args: Vec<String>) -> Option<path::PathBuf> {
    let input_csv_file = match args.get(1) {
        None => {
            // println!(
            //     "No input provided, please refer to the README.md \
            //     file for input examples"
            // );
            return None;
        }
        Some(file) => file,
    };

    let mut input_path = path::PathBuf::new();
    input_path.push(input_csv_file);

    if input_path.exists() {
        let file_extension = input_path.extension().and_then(OsStr::to_str);

        return if let Some("csv") = file_extension {
            Some(input_path)
        } else {
            // println!(
            //     "Input is NOT a CSV file please refer to the README.md \
            //     file for input example"
            // );
            None
        };
    } else {
        // println!(
        //     "Input file does NOT exist please refer to the README.md \");
        //     file for input examples"
        // );
    }

    None
}
