use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// Specify the ID of the spreadsheet
    #[structopt(long = "id")]
    pub spreasheet_id: String,
    /// If true, ignore csv header
    #[structopt(long)]
    pub ignore_header: bool,
    /// Specify the directory to output CSV files
    #[structopt(short, long, parse(from_os_str))]
    pub out_dir: Option<PathBuf>,
    /// Specify the path to the JSON file for the service account key
    #[structopt(short = "k", long = "key", parse(from_os_str))]
    pub service_account_key: Option<PathBuf>
}