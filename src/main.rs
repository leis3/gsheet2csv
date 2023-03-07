mod opt;

use std::io::Write;
use structopt::StructOpt;
use google_sheets4 as sheets4;
use sheets4::Sheets;
use sheets4::{hyper, hyper_rustls, oauth2};

#[tokio::main]
async fn main() {
    let opt = opt::Opt::from_args();

    let hub = {
        let secret = oauth2::read_service_account_key("service_account_key.json")
            .await
            .expect("failed to read service_account_key.json");

        let auth = oauth2::ServiceAccountAuthenticator::builder(secret)
            .build()
            .await
            .expect("failed to create authenticator");
        
        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build();

        let client = hyper::Client::builder().build(connector);

        Sheets::new(client, auth)
    };

    let titles = {
        let (_, ss) = hub.spreadsheets()
            .get(&opt.spreasheet_id)
            .doit().await
            .expect("failed tp get spreadsheet");
        ss.sheets.unwrap().into_iter()
            .map(|sheet| sheet.properties.unwrap().title.unwrap())
            .collect::<Vec<_>>()
    };

    for title in titles {
        let (_, value) = hub.spreadsheets().values_get(&opt.spreasheet_id, &title)
            .doit().await
            .expect("failed to get values");

        let values = {
            let mut values = value.values.unwrap();
            if opt.ignore_header { values.split_off(1) } else { values }
        };


        let mut file = std::fs::File::create(format!("{title}.csv")).unwrap();
        for e in values.into_iter().map(|v| v.join(",")) {
            writeln!(file, "{e}").unwrap();
        }
        file.flush().unwrap();
    }
}