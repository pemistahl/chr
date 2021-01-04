![chr](logo.png)

<br>

[![Build Status](https://github.com/pemistahl/chr/workflows/chr%20CI/badge.svg?branch=master)](https://github.com/pemistahl/chr/actions?query=workflow%3A%22chr+CI%22+branch%3Amaster)
[![dependency status](https://deps.rs/crate/chr/1.0.0/status.svg)](https://deps.rs/crate/chr/1.0.0)
[![lines of code](https://tokei.rs/b1/github/pemistahl/chr?category=code)](https://github.com/XAMPPRocky/tokei)
[![Downloads](https://img.shields.io/crates/d/chr.svg)](https://crates.io/crates/chr)

[![Crates.io](https://img.shields.io/crates/v/chr.svg)](https://crates.io/crates/chr)
[![Lib.rs](https://img.shields.io/badge/lib.rs-v1.0.0-blue)](https://lib.rs/crates/chr)
[![license](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)

[![Linux Download](https://img.shields.io/badge/Linux%20Download-v1.0.0-blue?logo=Linux)](https://github.com/pemistahl/chr/releases/download/v1.0.0/chr-v1.0.0-x86_64-unknown-linux-musl.tar.gz)
[![MacOS Download](https://img.shields.io/badge/macOS%20Download-v1.0.0-blue?logo=Apple)](https://github.com/pemistahl/chr/releases/download/v1.0.0/chr-v1.0.0-x86_64-apple-darwin.tar.gz)
[![Windows Download](https://img.shields.io/badge/Windows%20Download-v1.0.0-blue?logo=Windows)](https://github.com/pemistahl/chr/releases/download/v1.0.0/chr-v1.0.0-x86_64-pc-windows-msvc.zip)

## <a name="table-of-contents"></a> Table of Contents
1. [What does this tool do?](#what-does-tool-do)
2. [How to install?](#how-to-install)
3. [How to use?](#how-to-use)
4. [How to build?](#how-to-build)
5. [Alternatives](#alternatives)
6. [Do you want to contribute?](#contribution)

## 1. <a name="what-does-tool-do"></a> What does this tool do? <sup>[Top ▲](#table-of-contents)</sup>

*chr* is a command-line utility that is meant to give various information about Unicode characters.
Currently, this information includes a character's Unicode escape sequence and its name, block, 
category and age as stated in the [Unicode Character Database (UCD)](https://www.unicode.org/reports/tr44).
The UCD in its current version 13.0 is the main and only source of information for now. More properties
and sources will be added in later releases.

## 2. <a name="how-to-install"></a> How to install? <sup>[Top ▲](#table-of-contents)</sup>

You can download the self-contained executable for your platform above and put it in a place of your choice.

*chr* is also hosted on [crates.io](https://crates.io/crates/chr),
the official Rust package registry. If you are a Rust developer and already have the Rust
toolchain installed, you can install by compiling from source using
[*cargo*](https://doc.rust-lang.org/cargo/), the Rust package manager:

```
cargo install chr
```

Package managers such as [Scoop](https://scoop.sh) (for Windows) and 
[Homebrew](https://brew.sh) (for macOS and Linux) will be supported soon.

## 3. <a name="how-to-use"></a> How to use? <sup>[Top ▲](#table-of-contents)</sup>

When running the executable for the first time, an SQLite database that is bundled with it will be unzipped
to the current user's home directory. It can be found in a hidden directory under the path `<home>/.chr/chr_1_0_0.db`.
This database is queried each time *chr* is used. It is initially created when building the source code. 
Various UCD files are downloaded from the internet, the relevant information is copied into the SQLite database and 
then the database is zipped and included within the executable.

```
$ chr -h

chr 1.0.0
© 2021 Peter M. Stahl <pemistahl@gmail.com>
Licensed under the Apache License, Version 2.0
Downloadable from https://crates.io/crates/chr
Source code at https://github.com/pemistahl/chr

chr is a command-line tool that gives
information about Unicode characters.

USAGE:
    chr [FLAGS] <CHARS>... --name <NAME>

FLAGS:
        --no-paging    Disables paging for the terminal output
    -c, --colorize     Provides syntax highlighting for the terminal output
    -h, --help         Prints help information
    -v, --version      Prints version information

OPTIONS:
    -n, --name <NAME>    Searches for characters by their name as
                         stated in the Unicode Character Database

ARGS:
    <CHARS>...    One or more characters separated by blank space
```

The tool is mainly meant to search for information about specific characters.
All characters of interest, separated by blank space, can be given to *chr* at the same time.
The entries are sorted by their Unicode escape sequence in ascending order:

```
$ chr Ä @ $ ß !

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
```

It is also possible to search for characters by their official name in the UCD:

```
$ chr --name honey

>>> 2 results found

1.	🍯	U+1F36F
HONEY POT
Miscellaneous Symbols and Pictographs	Other Symbol
since 6.0

2.	🐝	U+1F41D
HONEYBEE
Miscellaneous Symbols and Pictographs	Other Symbol
since 6.0
```

Long result lists are paged automatically in supported terminals for easier browsing.
The [minus](https://github.com/arijit79/minus) crate is used for this purpose.
Its key controls are documented in a 
[subsection of the project's readme](https://github.com/arijit79/minus#end-user-help).
If the paging is not deactivated automatically in unsupported terminals or if it does 
not work as expected, it can be explicitly switched off using the `--no-paging` flag.
The result lists can be colorized as well with the `--colorize` flag which produces
nicer looking output in supported terminals.

## 4. <a name="how-to-build"></a> How to build? <sup>[Top ▲](#table-of-contents)</sup>

In order to build the source code yourself, you need the
[stable Rust toolchain](https://www.rust-lang.org/tools/install) installed on your machine
so that [*cargo*](https://doc.rust-lang.org/cargo/), the Rust package manager is available.

```
git clone https://github.com/pemistahl/chr.git
cd chr
cargo build
```

The source code is accompanied by some integration tests. For running them, simply say:

```
cargo test
```

## 5. <a name="alternatives"></a> Alternatives <sup>[Top ▲](#table-of-contents)</sup>

An alternative tool named [*cha(rs)*](https://github.com/antifuchs/chars) already exists that targets
a similar purpose. Currently, it offers more character properties than *chr*. Unfortunately, it lacks 
a proper command-line interface, colorization, and the presented information is not neatly arranged, 
in my opinion. It also bundles all the UCD files in its repository which is not necessary and impedes 
maintenance and future updates.

With *chr*, only the url to the new UCD version needs to be updated. Assuming that the format of the UCD 
files does not change between versions, this is all there is to it to provide *chr* with the newest data.
The data presentation is focused on readability and concentrates on the most essential properties for now.
Last but not least, the fact that *chr* is backed by an SQL database allows for both complex and performant
queries, especially in later releases when the functionality gets extended.

## 6. <a name="contribution"></a> Do you want to contribute? <sup>[Top ▲](#table-of-contents)</sup>

In case you want to contribute something to *chr* even though it's in a very early stage of development,
then I encourage you to do so nevertheless. Do you have ideas for cool features? Or have you found any bugs so far?
Feel free to open an issue or send a pull request. It's very much appreciated. :-)