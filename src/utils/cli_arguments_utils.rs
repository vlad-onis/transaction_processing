use std::ffi::OsStr;
use std::io;
use std::path;

/// Returns a path to the csv file if there is a valid csv file, None otherwise.
/// # Arguments
/// * args - Vector of command line arguments.
pub fn parse_input(args: Vec<String>) -> Result<path::PathBuf, io::Error> {
    let input_csv_file = match args.get(1) {
        None => {
            return Err(io::Error::new(io::ErrorKind::NotFound, "No input provided"));
        }
        Some(file) => file,
    };

    let mut input_path = path::PathBuf::new();
    input_path.push(input_csv_file);

    if input_path.exists() {
        let file_extension = input_path.extension().and_then(OsStr::to_str);

        return if let Some("csv") = file_extension {
            Ok(input_path)
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Input is not a csv file",
            ));
        };
    } else {
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Input file does not exist",
    ))
}
