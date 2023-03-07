# gsheet2csv

Retrieves data from the specified spreadsheet and outputs it as a CSV file. The CSV file name is saved with the spreadsheet sheet name.

## Installation

```console
$ cargo install --git https://github.com/leis3/gsheet2csv.git
```

## Usage

```console
$ gsheet2csv --id "spreadsheet_id" --ignore-header --out-dir "csv" --key
```
## Options

- `--id`
Specify the ID of the spreadsheet from which the data will be retrieved.
- `--ignore-header`
If this flag is specified, the first line is ignored and the second and subsequent lines are output to the CSV file.
- `--out-dir`
Specify the directory to output CSV files.
- `--key`
Specify the path to the JSON file for the service account key. JSON files are stored under "gsheet2csv" directory in the cache directory.