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

use assert_cmd::prelude::*;
use indoc::indoc;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn succeeds_with_character_search_option() {
    let mut chr = init_command();
    chr.args(&["--no-paging", "Ä", "@", "$", "ß", "!"]);
    chr.assert()
        .success()
        .stdout(predicate::str::contains(indoc!(
            "
            1.	!	U+0021
            EXCLAMATION MARK
            Basic Latin	Other Punctuation
            since 1.1

            2.	$	U+0024
            DOLLAR SIGN
            Basic Latin	Currency Sign
            since 1.1

            3.	@	U+0040
            COMMERCIAL AT
            Basic Latin	Other Punctuation
            since 1.1

            4.	Ä	U+00C4
            LATIN CAPITAL LETTER A WITH DIAERESIS
            Latin-1 Supplement	Uppercase Letter
            since 1.1

            5.	ß	U+00DF
            LATIN SMALL LETTER SHARP S
            Latin-1 Supplement	Lowercase Letter
            since 1.1
            "
        )));
}

#[test]
fn succeeds_with_name_search_option() {
    let mut chr = init_command();
    chr.args(&["--no-paging", "--name", "honey"]);
    chr.assert()
        .success()
        .stdout(predicate::str::contains(indoc!(
            "
            1.	🍯	U+1F36F
            HONEY POT
            Miscellaneous Symbols and Pictographs	Other Symbol
            since 6.0

            2.	🐝	U+1F41D
            HONEYBEE
            Miscellaneous Symbols and Pictographs	Other Symbol
            since 6.0
            "
        )));
}

#[test]
fn fails_with_string_instead_of_chars() {
    let mut chr = init_command();
    chr.args(&["Ä@"]);
    chr.assert().failure().stderr(predicate::str::contains(
        "Invalid value for '<CHARS>...': too many characters in string",
    ));
}

#[test]
fn fails_with_both_character_and_name_search_option() {
    let mut chr = init_command();
    chr.args(&["Ä@", "--name", "honey"]);
    chr.assert().failure().stderr(predicate::str::contains(
        "error: The argument '--name <NAME>' cannot be used with 'chars'",
    ));
}

fn init_command() -> Command {
    Command::cargo_bin("chr").unwrap()
}
