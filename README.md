# metadata-transferer

A simple CLI tool to transfer metadata between files.

Built in rust.

## Usage

Install using cargo

```bash
cargo install metadata-transferer
```

Then provide an input and an output

```bash
metadata-transferer <input> <output>
```

Paths can be either files or directories. For each file in the input directory it searches for a file with the same filename (ingoring extensions) in the output directory. Keep in mind that if you pass single files they get interpreted as "directories" with one file, so the names still have to match. 