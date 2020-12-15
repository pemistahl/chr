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
use rusqlite::{Connection, Row, ToSql, NO_PARAMS};
use std::char;
use std::fs::{create_dir, File};
use std::io::{Cursor, Read, Write};
use std::iter;
use std::path::Path;
use structopt::StructOpt;
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
    #[structopt(
        value_name = "CHARS",
        required_unless = "name",
        conflicts_with = "name",
        help = "One or more characters separated by blank space"
    )]
    chars: Vec<char>,

    // --------------------
    // FLAGS
    // --------------------
    #[structopt(
        name = "paging",
        short,
        long,
        help = "Enables terminal paging for large result tables",
        long_help = "Enables terminal paging for large result tables.\n\
                     The table can be scrolled through using the 'Arrow Up' and 'Arrow Down' keys."
    )]
    is_paging_enabled: bool,

    // --------------------
    // OPTIONS
    // --------------------
    #[structopt(
        name = "name",
        value_name = "NAME",
        short,
        long,
        required_unless = "chars",
        help = "Searches for characters by their name as\n\
                stated in the Unicode Character Database"
    )]
    name: Option<String>,
}

fn main() {
    let cli: CLI = CLI::from_args();
    let database = connect_to_database();
    let results = search_database(database, &cli);

    render(results, &cli);
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

fn search_database(database: Connection, cli: &CLI) -> Vec<String> {
    let mut sql = String::from("SELECT * FROM UnicodeData WHERE ");

    if !cli.chars.is_empty() {
        let chars_as_decimals = convert_chars_to_decimals(&cli.chars);
        let params = iter::repeat("?")
            .take(chars_as_decimals.len())
            .collect::<Vec<_>>()
            .join(",");

        sql.push_str(&format!("codepoint IN ({})", params));
        retrieve_results(database, sql, chars_as_decimals)
    } else {
        sql.push_str(&format!("name LIKE '%{}%'", cli.name.as_ref().unwrap()));
        retrieve_results(database, sql, NO_PARAMS)
    }
}

fn retrieve_results<P>(database: Connection, sql: String, params: P) -> Vec<String>
where
    P: IntoIterator,
    P::Item: ToSql,
{
    let mut statement = database.prepare(&sql).unwrap();
    let mut rows = statement.query(params).unwrap();
    let mut results = vec![];

    while let Some(row) = rows.next().unwrap() {
        let result = convert_database_row_to_result(row);
        results.push(result);
    }

    results
}

fn convert_database_row_to_result(row: &Row) -> String {
    let codepoint_column_index = row.column_index("codepoint").unwrap();
    let name_column_index = row.column_index("name").unwrap();

    let c = char::from_u32(row.get_unwrap(codepoint_column_index)).unwrap();
    let hex_code = format!("U+{:04x}", to_decimal_number(c)).to_uppercase();
    let name: String = row.get_unwrap(name_column_index);

    format!("{}\t{}\n{}", c, hex_code, name)
}

fn convert_chars_to_decimals(chars: &[char]) -> Vec<u32> {
    chars.iter().map(|&c| to_decimal_number(c)).collect()
}

fn to_decimal_number(c: char) -> u32 {
    u32::from_str_radix(&to_hex_code(c), 16).expect("Could not convert hex to decimal number")
}

fn to_hex_code(c: char) -> String {
    let escaped_char = c.escape_unicode().to_string();
    escaped_char[3..escaped_char.len() - 1].to_string()
}

fn render(mut results: Vec<String>, cli: &CLI) {
    if results.len() >= 10 {
        let summary = format!("### {} results found ###", results.len());
        results.insert(0, summary.clone());
        results.push(summary);
    }

    let result = results.join("\n\n");

    if cli.is_paging_enabled {
        minus::page_all(result);
    } else {
        println!("{}", result);
    }
}
