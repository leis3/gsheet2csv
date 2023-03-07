# gsheet2csv

`gsheet2csv` is a command-line tool that retrieves data from a Google spreadsheet and outputs it as a CSV file. The name of each CSV file is the same as the name of the corresponding sheet in the spreadsheet.

## Installation

```console
$ cargo install --git https://github.com/leis3/gsheet2csv.git
```

## Usage

To retrieve data from a spreadsheet, use the following command:

```console
$ gsheet2csv --id "spreadsheet_id" --ignore-header --out-dir "csv" --key "./service_account_key.json"
```

You can specify the following options to customize the output:

- `--id`: Specify the ID of the spreadsheet from which the data will be retrieved.
- `--ignore-header`: If this flag is specified, the first line is ignored and the second and subsequent lines are output to the CSV file.
- `--out-dir`: Specify the directory to output CSV files.
- `--key`: Specify the path to the JSON file for the service account key. JSON files are stored under "gsheet2csv" directory in the cache directory.

## Assumptions

This tool assumes that the Google Sheets API has been enabled and that you have a service account with a JSON key file.
