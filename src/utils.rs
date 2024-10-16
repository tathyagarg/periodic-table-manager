use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn get_tables(source_file: &PathBuf) -> Vec<String> {
    let mut file = File::open(source_file).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    let json_data = json::parse(&contents).unwrap();

    let mut curr;

    let mut tables = Vec::new();

    for entry in json_data.entries() {
        (curr, _) = entry;
        tables.push(String::from(curr));
    }

    tables
}

pub fn wrap(text: &String, opts: &textwrap::Options) -> Vec<String> {
    let mut res = Vec::new();

    for elem in textwrap::wrap(text.as_str(), opts) {
        res.push(elem.to_string());
    }

    res
}
