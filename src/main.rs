use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::fs::File;
use csv::Writer;

#[derive(Serialize, Deserialize, Debug)]
struct Location {
    x: i32,
    y: i32,
    z: i32,
    date: String,  // Assuming date is in ISO 8601 format as a string
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    // Adjust the URL if needed to ensure date information is included in the API response
    let url = "https://api.openf1.org/v1/location?session_key=9149&driver_number=1";

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let locations: Vec<Location> = response.json().await?;
        write_to_csv(locations)?;
        println!("Data has been written to zandvoort_data.csv");
    } else {
        println!("Failed to get data: {}", response.status());
    }

    Ok(())
}

fn write_to_csv(locations: Vec<Location>) -> Result<(), Box<dyn StdError>> {
    let file = File::create("zandvoort_data.csv")?;
    let mut wtr = Writer::from_writer(file);

    // Add 'Date' to the header
    //wtr.write_record(&["Date", "X", "Y", "Z"])?;

    for location in locations {
        wtr.serialize(&location)?;
    }
    wtr.flush()?;
    Ok(())
}
