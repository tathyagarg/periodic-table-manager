// #[macro_use]
extern crate json;
extern crate termion;
extern crate textwrap;

use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::iter::zip;
use std::path::PathBuf;
use std::vec::Vec;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod colors;
mod commands;
mod notes;
mod table;

fn get_tables(source_file: &PathBuf) -> Vec<String> {
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

fn get_button(table_name: &String, direction: String) -> String {
    let mut buffer = String::new();

    buffer.push_str("╭──────────────────────────╮\n");
    buffer.push_str(format!("│ {: ^24} │\n", direction).as_str());
    let name = if table_name.len() > 26 {
        &format!("{}...", &table_name.to_string()[0..21])
    } else {
        table_name
    };

    buffer.push_str(format!("│ {: ^24} │\n", name).as_str());
    buffer.push_str("╰──────────────────────────╯\n");

    buffer
}

fn get_button_row(prev_button: String, next_button: String) -> String {
    let mut buffer = String::new();
    for (l1, l2) in zip(prev_button.split('\n'), next_button.split('\n')) {
        buffer.push_str(format!("{}{}\r\n", l1, l2).as_str());
    }

    buffer.pop();
    buffer.pop();
    buffer
}

fn main() {
    // -------------- INITIALIZATION --------------
    let source_file: PathBuf = PathBuf::from("data.json");
    let mut nr = notes::NotesReader::new(PathBuf::from("elements.json"));
    let mut temp_buffer: String = String::new();
    let mut buffer: String = String::new();

    let table_names: Vec<String> = get_tables(&source_file);
    let mut tables: Vec<table::models::Table> = Vec::new();
    let mut table_count = 0;

    for table_name in table_names.clone() {
        tables.push(table::models::Table::new(source_file.clone(), table_name));
        table_count += 1;
    }

    let mut curr: usize = 0;

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    stdout.flush().unwrap();

    let mut prev_button: String = get_button(
        &String::from(table_names.last().unwrap()),
        String::from("<="),
    );
    let mut next_button: String = get_button(&table_names[1], String::from("=>"));

    write!(
        stdout,
        "{}{}{}\r\n",
        tables[curr].display(),
        get_button_row(prev_button, next_button),
        buffer
    )
    .unwrap();

    // -------------- EVENT LOOP --------------
    for k in stdin.keys() {
        match k.as_ref().unwrap() {
            Key::Ctrl('c') => break,
            Key::Char('\n') => {
                buffer = temp_buffer.clone();
                temp_buffer = String::new();
                buffer = commands::parse_command(&mut nr, &buffer);
            }
            Key::Char(letter) => {
                buffer = String::new();
                temp_buffer.push(*letter);
            }
            Key::Backspace => {
                temp_buffer.pop();
            }
            Key::Left => curr = if curr == 0 { table_count - 1 } else { curr - 1 },
            Key::Right => curr = (curr + 1) % table_count,
            _ => {}
        }

        prev_button = get_button(
            &table_names[if curr == 0 { table_count - 1 } else { curr - 1 }],
            String::from("<="),
        );

        next_button = get_button(&table_names[(curr + 1) % table_count], String::from("=>"));

        write!(
            stdout,
            "{}{}{}{}{}{}\r\n",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            tables[curr].display(),
            get_button_row(prev_button, next_button),
            temp_buffer,
            buffer
        )
        .unwrap();

        stdout.flush().unwrap();
    }
}
