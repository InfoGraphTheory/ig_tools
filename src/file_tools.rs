
use std::fs;

use crate::list_tools;

///
/// Appeds the argument line to the file file_name 
///
pub fn append_ln_to_file(line: &str, file_name: &str) {
    let lines = fs::read_to_string(file_name).unwrap();
    let text = list_tools::append_ln(line, &lines);
    let _ = fs::write(file_name, text);
}

///
/// Write argument text to file in argument file_name.
///
pub fn write(file_name: String, text: &str) {
    let _ = fs::write(file_name.as_str(), text);
}


