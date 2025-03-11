
use std::{fs, path::PathBuf};

use crate::list_tools;

///
/// Appeds the argument line to the file file_name 
///
pub fn append_ln_to_file(line: &str, file_name: PathBuf) {
    let lines = fs::read_to_string(file_name.clone()).unwrap();
    let text = list_tools::append_ln(line, &lines);
    let _ = fs::write(file_name.clone(), text);
}

///
/// Write argument text to file in argument file_name.
///
pub fn write(file_name: PathBuf, text: &str) {
    let _ = fs::write(file_name.clone(), text);
}


