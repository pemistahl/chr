/*
 * Copyright © 2021 Peter M. Stahl pemistahl@gmail.com
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

use csv::{Reader, ReaderBuilder};
use reqwest::blocking::Client;
use rusqlite::{params, Connection, Error, NO_PARAMS};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::env::var_os;
use std::fs::File;
use std::io::Write;
use std::io::{BufReader, Read};
use std::ops::RangeInclusive;
use std::option::Option::Some;
use std::path::Path;
use zip::write::FileOptions;
use zip::ZipWriter;

const UCD_URL: &str = "http://ftp.unicode.org/Public/13.0.0/ucd";

const BLOCKS_FILE_NAME: &str = "Blocks.txt";
const DERIVED_AGE_FILE_NAME: &str = "DerivedAge.txt";
const UNICODE_DATA_FILE_NAME: &str = "UnicodeData.txt";
const DATABASE_FILE_NAME: &str = "chr.db";
const ZIP_FILE_NAME: &str = "chr.db.zip";

fn main() {
    let out_dir = var_os("OUT_DIR").unwrap();
    let target_directory_path = Path::new(&out_dir);

    download_files(target_directory_path);

    let mut unicode_char_data_map = process_unicode_data_file(target_directory_path);

    process_blocks_file(target_directory_path, &mut unicode_char_data_map);
    process_derived_age_file(target_directory_path, &mut unicode_char_data_map);

    save_to_database(target_directory_path, unicode_char_data_map);
    compress_database(target_directory_path);
}

fn download_files(target_directory_path: &Path) {
    let ucd_base_url = Path::new(UCD_URL);
    let file_urls = vec![
        ucd_base_url.join(BLOCKS_FILE_NAME),
        ucd_base_url.join(DERIVED_AGE_FILE_NAME),
        ucd_base_url.join(UNICODE_DATA_FILE_NAME),
    ];
    let client = Client::new();

    for file_url in file_urls {
        let file_path = target_directory_path.join(file_url.file_name().unwrap());

        if !file_path.exists() {
            let mut file_data = client
                .get(file_url.to_str().unwrap())
                .send()
                .expect("File download failed");
            let mut file =
                File::create(file_path).expect("New file could not be created at the given path");

            file_data
                .copy_to(&mut file)
                .expect("Downloaded file data could not be written to disk");
        }
    }
}

fn process_unicode_data_file(target_directory_path: &Path) -> BTreeMap<u32, UnicodeCharData> {
    let mut csv_file_reader = open_csv_file_reader(target_directory_path, UNICODE_DATA_FILE_NAME);
    let mut csv_row_iterator = csv_file_reader.deserialize::<UnicodeDataFileRow>();
    let mut unicode_char_data_map = BTreeMap::<u32, UnicodeCharData>::new();

    while let Some(row) = csv_row_iterator.next() {
        let unicode_data_file_row = row.expect("UnicodeDataFileRow could not be deserialized");
        let codepoint = to_decimal_number(&unicode_data_file_row.hexcode);

        if unicode_data_file_row.name.ends_with("First>") {
            let name = &unicode_data_file_row.name.split(',').next().unwrap()[1..];
            let next_row = csv_row_iterator.next().unwrap().unwrap();
            let last_codepoint = to_decimal_number(&next_row.hexcode);

            for point in codepoint..=last_codepoint {
                let unicode_char_data = UnicodeCharData::from(&unicode_data_file_row, point, name);
                unicode_char_data_map.insert(point, unicode_char_data);
            }
        } else {
            let unicode_char_data = UnicodeCharData::from(
                &unicode_data_file_row,
                codepoint,
                &unicode_data_file_row.name,
            );
            unicode_char_data_map.insert(codepoint, unicode_char_data);
        }
    }

    unicode_char_data_map
}

fn process_blocks_file(
    target_directory_path: &Path,
    unicode_char_data_map: &mut BTreeMap<u32, UnicodeCharData>,
) {
    let mut csv_file_reader = open_csv_file_reader(target_directory_path, BLOCKS_FILE_NAME);

    for result in csv_file_reader.records() {
        let row = result.expect("CSV row could not be unwrapped");

        if row.len() == 2 {
            let codepoints = row.get(0).unwrap().trim();
            let block_name = row.get(1).unwrap().trim();

            if !codepoints.starts_with('#') {
                for codepoint in codepoint_range(codepoints) {
                    if unicode_char_data_map.contains_key(&codepoint) {
                        unicode_char_data_map.get_mut(&codepoint).unwrap().block =
                            block_name.to_string();
                    }
                }
            }
        }
    }
}

fn process_derived_age_file(
    target_directory_path: &Path,
    unicode_char_data_map: &mut BTreeMap<u32, UnicodeCharData>,
) {
    let mut csv_file_reader = open_csv_file_reader(target_directory_path, DERIVED_AGE_FILE_NAME);

    for result in csv_file_reader.records() {
        let row = result.expect("CSV row could not be unwrapped");

        if row.len() == 2 {
            let codepoints = row.get(0).unwrap().trim();
            let unicode_version = row.get(1).unwrap().split('#').next().unwrap().trim();

            if !codepoints.starts_with('#') {
                for codepoint in codepoint_range(codepoints) {
                    if unicode_char_data_map.contains_key(&codepoint) {
                        unicode_char_data_map.get_mut(&codepoint).unwrap().age =
                            unicode_version.to_string();
                    }
                }
            }
        }
    }
}

fn save_to_database(
    target_directory_path: &Path,
    unicode_char_data_map: BTreeMap<u32, UnicodeCharData>,
) {
    let database_path = target_directory_path.join(DATABASE_FILE_NAME);
    let database = Connection::open(database_path).expect("Database could not be created");

    database
        .execute(
            r#"
            CREATE TABLE IF NOT EXISTS UnicodeData (
                codepoint INTEGER NOT NULL PRIMARY KEY,
                name TEXT NOT NULL,
                category TEXT NOT NULL,
                block TEXT NOT NULL,
                age TEXT NOT NULL
            ) WITHOUT ROWID;
            "#,
            NO_PARAMS,
        )
        .expect("Database table could not be created");

    let entry_count: Result<u32, Error> =
        database.query_row("SELECT COUNT(*) FROM UnicodeData", NO_PARAMS, |row| {
            row.get(0)
        });

    if entry_count.unwrap() > 0 {
        return;
    }

    let mut insert_statement = database
        .prepare_cached(
            r#"INSERT INTO UnicodeData VALUES (
                ?,?,?,?,?
            )"#,
        )
        .unwrap();

    for (codepoint, char_data) in unicode_char_data_map.iter() {
        insert_statement
            .execute(params![
                *codepoint,
                &char_data.name,
                &char_data.category,
                &char_data.block,
                &char_data.age,
            ])
            .expect("Database insert statement failed");
    }
}

fn compress_database(target_directory_path: &Path) {
    let database_path = target_directory_path.join(DATABASE_FILE_NAME);
    let database_file =
        File::open(database_path).expect("Database file could not be opened for compression");
    let mut database_reader = BufReader::new(database_file);
    let mut database = vec![];

    database_reader
        .read_to_end(&mut database)
        .expect("Database could not be read as bytes");

    let zip_file_path = target_directory_path.join(ZIP_FILE_NAME);
    let zip_file = File::create(zip_file_path).expect("Empty zip file could not be created");
    let mut zip = ZipWriter::new(zip_file);

    zip.start_file(DATABASE_FILE_NAME, FileOptions::default())
        .expect("Zip content file could not be created");
    zip.write_all(&database)
        .expect("Database could not be written to zip file");
}

fn open_csv_file_reader(target_directory_path: &Path, file_name: &str) -> Reader<File> {
    ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .flexible(true)
        .from_path(target_directory_path.join(file_name))
        .unwrap_or_else(|_| panic!("File {} could not be opened for reading", file_name))
}

fn to_decimal_number(hexcode: &str) -> u32 {
    u32::from_str_radix(hexcode, 16).expect("Could not convert hex to decimal number")
}

fn codepoint_range(codepoints: &str) -> RangeInclusive<u32> {
    if codepoints.contains("..") {
        let start_and_end = codepoints
            .split("..")
            .map(|hexcode| to_decimal_number(hexcode))
            .collect::<Vec<_>>();
        let start = *start_and_end.get(0).unwrap();
        let end = *start_and_end.get(1).unwrap();
        start..=end
    } else {
        let start = to_decimal_number(codepoints);
        start..=start
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct UnicodeDataFileRow {
    hexcode: String,
    name: String,
    category: String,
    canonical_combining_class: u32,
    bidi_class: String,
    decomposition_type: Option<String>,
    numeric_value_1: Option<u32>,
    numeric_value_2: Option<u32>,
    numeric_value_3: Option<String>,
    bidi_mirrored: String,
    unicode_1_name: Option<String>,
    iso_comment: Option<String>,
    uppercase_mapping: Option<String>,
    lowercase_mapping: Option<String>,
    titlecase_mapping: Option<String>,
}

#[derive(Default)]
struct UnicodeCharData {
    codepoint: u32,
    name: String,
    category: String,
    block: String,
    age: String,
}

impl UnicodeCharData {
    fn from(unicode_data_file_row: &UnicodeDataFileRow, codepoint: u32, name: &str) -> Self {
        let mut unicode_char_data = UnicodeCharData::default();
        unicode_char_data.codepoint = codepoint;
        unicode_char_data.name = name.to_string();
        unicode_char_data.category = unicode_data_file_row.category.clone();
        unicode_char_data
    }
}
