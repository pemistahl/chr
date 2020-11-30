/*
 * Copyright © 2020 Peter M. Stahl pemistahl@gmail.com
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use dirs::home_dir;
use rusqlite::{Connection, Rows, Statement};
use std::char;
use std::fs::{create_dir, File};
use std::io::{Cursor, Read, Write};
use std::iter;
use std::path::Path;
use structopt::StructOpt;
use term_table::row::Row;
use term_table::table_cell::TableCell;
use term_table::{Table, TableStyle};
use zip::ZipArchive;

const DATABASE_DIRECTORY_NAME: &str = ".chr";
const DATABASE_FILE_NAME: &str = "chr_1_0_0.db";

#[derive(StructOpt)]
#[structopt(
    author = "© 2020 Peter M. Stahl <pemistahl@gmail.com>",
    about = "Licensed under the Apache License, Version 2.0\n\
             Downloadable from https://crates.io/crates/chr\n\
             Source code at https://github.com/pemistahl/chr\n\n\
             chr is a command-line tool that prints useful\n\
             information about any Unicode character.",
    version_short = "v"
)]
struct CLI {
    // --------------------
    // ARGS
    // --------------------
    #[structopt(value_name = "CHARS", required = true)]
    chars: Vec<char>,
}

fn main() {
    let cli: CLI = CLI::from_args();
    let database = connect_to_database();
    let chars_as_decimals = convert_chars_to_decimals(&cli.chars);
    let mut statement = prepare_database_statement(&database, chars_as_decimals.len());
    let rows = statement
        .query(chars_as_decimals)
        .expect("Query parameters are invalid");
    let table = prepare_terminal_table(rows);

    println!("{}", table.render());
}

fn connect_to_database() -> Connection {
    let home_directory = home_dir().expect("Home directory could not be found");
    let database_file_path = home_directory
        .join(DATABASE_DIRECTORY_NAME)
        .join(DATABASE_FILE_NAME);

    if !database_file_path.is_file() {
        println!("Preparing Unicode character database, please wait a moment...");
        unzip_database(&home_directory);
        println!("Database is ready\n");
    }

    Connection::open(database_file_path).expect("Database connection could not be established")
}

fn unzip_database(home_directory: &Path) {
    let zip_file_bytes = include_bytes!(concat!(env!("OUT_DIR"), "/chr.db.zip"));
    let zip_file_reader = Cursor::new(zip_file_bytes);
    let mut zip_archive =
        ZipArchive::new(zip_file_reader).expect("Zip archive could not be opened");
    let mut zip_file_content = zip_archive
        .by_index(0)
        .expect("Database could not be found within zip archive");
    let mut database_file_bytes = vec![];

    zip_file_content
        .read_to_end(&mut database_file_bytes)
        .expect("Bytes of database file could not be read");

    let database_directory = home_directory.join(DATABASE_DIRECTORY_NAME);

    if !database_directory.is_dir() {
        create_dir(&database_directory).expect("Database directory could not be created");
    }

    let database_file_path = database_directory.join(DATABASE_FILE_NAME);
    let mut database_file =
        File::create(database_file_path).expect("Database file could not be created");

    database_file
        .write_all(&database_file_bytes)
        .expect("Database content could not be written to file");
}

fn convert_chars_to_decimals(chars: &Vec<char>) -> Vec<u32> {
    chars.iter().map(|&c| to_decimal_number(c)).collect()
}

fn to_hex_code(c: char) -> String {
    let escaped_char = c.escape_unicode().to_string();
    escaped_char[3..escaped_char.len() - 1].to_uppercase()
}

fn to_decimal_number(c: char) -> u32 {
    u32::from_str_radix(&to_hex_code(c), 16).expect("Could not convert hex to decimal number")
}

fn prepare_database_statement(database: &Connection, param_count: usize) -> Statement {
    let params = iter::repeat("?")
        .take(param_count)
        .collect::<Vec<_>>()
        .join(",");
    let sql = format!(
        r#"
        SELECT
            codepoint,
            name
        FROM
            UnicodeData
        WHERE
            codepoint IN ({})
        "#,
        params
    );
    database.prepare(&sql).unwrap()
}

fn prepare_terminal_table(mut rows: Rows) -> Table {
    let mut table = Table::new();
    table.style = TableStyle::rounded();
    table.add_row(create_table_row(vec!["Char", "Codepoint", "Name"]));

    while let Some(db_row) = rows.next().unwrap() {
        let c = char::from_u32(db_row.get_unwrap(0)).unwrap();
        let hex_code = format!("U+{:04x}", to_decimal_number(c)).to_uppercase();
        let name: String = db_row.get_unwrap(1);
        table.add_row(create_table_row(vec![&c.to_string(), &hex_code, &name]));
    }

    table
}

fn create_table_row(columns: Vec<&str>) -> Row<'static> {
    let table_cells = columns
        .iter()
        .map(|column| TableCell::new(column))
        .collect::<Vec<_>>();

    Row::new(table_cells)
}
