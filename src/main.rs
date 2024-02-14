use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
struct LogEntry {
    clientaddr: String,
    destaddr: String,
    firewall: String,
    port: String,
    proto: String,
    action: String,
    // Add country field here
    country: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {

    let client = Client::new();
    let api_endpoint = "http://ip-api.com/json/";

    // Read CSV file
    let file = File::open("vernlog.csv")?;
    let reader = BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_reader(reader);

    for result in rdr.deserialize() {
        let mut record: LogEntry = result?;

        let response = client.get(&format!("{}{}", api_endpoint, record.clientaddr)).send()?;
        if response.status().is_success() {
            let json_data: serde_json::Value = response.json()?;
            if let Some(country_code) = json_data.get("countryCode").and_then(|c| c.as_str()) {
                record.country = Some(country_code.to_string());
            }
        }
        

        let json_string = serde_json::to_string(&record)?;
        println!("{}", json_string);
    }

    Ok(())
}