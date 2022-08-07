# metadata-transferer

A simple CLI tool to transfer metadata between files.

Written in rust.

## Usage

Install using cargo

```bash
cargo install metadata-transferer
```

Then provide an input and an output

```bash
metadata-transferer [OPTIONS] <INPUT> <OUTPUT>
```

You can get more help by typing the help command

```
metadata-transferer --help


metadata-transferer 0.2.0
CLI tool to transfer metadata between files

USAGE:
    metadata-transferer [OPTIONS] <INPUT> <OUTPUT>

ARGS:
    <INPUT>     Input path (reference, doesn't get affected)
    <OUTPUT>    Output path (DOES GET AFFECTED)

OPTIONS:
    -h, --help
            Print help information

    -m, --match-extension <match-extension>
            Tries to match the extension of files [default: false]

    -v, --verbose <verbose>
            Prints more info [default: true]

    -V, --version
            Print version information
```

## Notes 

Paths can be either files or directories. For each file in the input directory it searches for a file with the same filename (ingoring extensions) in the output directory. Keep in mind that if you pass single files they get interpreted as "directories" with one file, so the names still have to match. 