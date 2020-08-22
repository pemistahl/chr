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
use rusqlite::Connection;
use std::fs::{create_dir, File};
use std::io::{Cursor, Read, Write};
use std::path::Path;
use zip::ZipArchive;

const DATABASE_DIRECTORY_NAME: &str = ".chr";
const DATABASE_FILE_NAME: &str = "chr_1_0_0.db";

fn main() {
    let zip_file_bytes = include_bytes!(concat!(env!("OUT_DIR"), "/chr.db.zip"));
    let home_directory = home_dir().expect("Home directory could not be found");
    let database_file_path = home_directory
        .join(DATABASE_DIRECTORY_NAME)
        .join(DATABASE_FILE_NAME);

    if !database_file_path.is_file() {
        unzip_database(zip_file_bytes, &home_directory);
    }

    let database = connect_to_database(&database_file_path);
}

fn connect_to_database(database_file_path: &Path) -> Connection {
    Connection::open(database_file_path).expect("Database connection could not be established")
}

fn unzip_database(zip_file_bytes: &[u8], home_directory: &Path) {
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

    let database_directory = home_directory.join(".chr");

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
