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

use reqwest::blocking::get;
use std::env::var_os;
use std::fs::{create_dir, File};
use std::path::Path;

fn main() {
    let out_dir = var_os("OUT_DIR").unwrap();
    let assets_dir = Path::new(&out_dir).join("assets");

    if !assets_dir.exists() {
        create_dir(&assets_dir);
    }

    let ucd_url = Path::new("http://ftp.unicode.org/Public/13.0.0/ucd");
    let ucd_file_names = vec!["Blocks.txt", "DerivedAge.txt", "UnicodeData.txt"];

    for file_name in ucd_file_names {
        let file_path = assets_dir.join(file_name);

        if !file_path.exists() {
            let download_url = ucd_url.join(file_name);
            let mut file_data = get(download_url.to_str().unwrap()).unwrap();
            let mut file = File::create(file_path).unwrap();

            file_data.copy_to(&mut file);
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
}
