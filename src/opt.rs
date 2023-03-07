use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// ID of the spreadsheet
    #[structopt(long = "id")]
    pub spreasheet_id: String,
    /// If true, ignore csv header
    #[structopt(long)]
    pub ignore_header: bool
}