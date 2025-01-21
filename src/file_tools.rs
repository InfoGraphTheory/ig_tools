
use std::fs;

use crate::list_tools;

pub fn append_ln_to_file(line: &str, file_name: &str) {
//println!("Filename:{:?}",file_name);
//println!("line:{:?}",line);
    let lines = fs::read_to_string(file_name).unwrap();
    let text = list_tools::append_ln(line, &lines);
    let _ = fs::write(file_name, text);
}

pub fn sort_and_append_ln_to_file(line: &str, file_name: &str) {
     let lines = fs::read_to_string(file_name).unwrap();
    let text = list_tools::append_ln(line, &lines);
    let _ = fs::write(file_name, text);
}

pub fn write(file_name: String, text: &str) {
    let _ = fs::write(file_name.as_str(), text);
}


