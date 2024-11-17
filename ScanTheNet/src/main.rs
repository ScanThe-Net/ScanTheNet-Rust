use reqwest::blocking::get;
use reqwest::Error;
use serde::Deserialize;
use std::env;

const MAX_ENTRIES_DEFAULT: usize = 100;
const API_URL: &str = "https://api.scanthe.net/";

#[derive(Deserialize)]
struct Packet {
    id: String,
    timestamp: String,
    source_ip: String,
    source_port: String,
    dest_port: String,
    data: String,
}

#[derive(Deserialize)]
struct Response {
    data: Vec<Packet>,
}

fn print_logo() {
    println!(r#"
  _______                    _______ __           ____ __         __
 |     __|.----.---.-.----- |_     _|  |--.-----.|    |  |.-----.|  |_
 |__     ||  __|  _  |     |  |   | |     |  -__||       ||  -__||   _|
 |_______||____|___._|__|__|  |___| |__|__|_____||__|____||_____||____|
             
"#);
}

fn fetch_data(url: &str) -> Result<Response, Error> {
    let response: Response = get(url)?.json()?;
    Ok(response)
}

fn main() {
    print_logo();

    // Determine how many entries to display (default to max 100)
    let max_entries = if let Some(arg) = env::args().nth(1) {
        match arg.parse::<usize>() {
            Ok(num) if num >= 1 && num <= 100 => num,
            _ => {
                eprintln!("Please enter a number between 1 and 100.");
                std::process::exit(1);
            }
        }
    } else {
        MAX_ENTRIES_DEFAULT
    };

    // Fetch the data
    let response = fetch_data(API_URL).unwrap_or_else(|err| {
        eprintln!("Error fetching data: {}", err);
        std::process::exit(1);
    });

    // Display the relevant parts of the JSON
    for (i, packet) in response.data.iter().enumerate() {
        if i >= max_entries {
            break; // Stop if we reach the maximum entries
        }
        println!("ID: {}", packet.id);
        println!("Timestamp: {}", packet.timestamp);
        println!("Source IP: {}", packet.source_ip);
        println!("Source Port: {}", packet.source_port);
        println!("Destination Port: {}", packet.dest_port);
        println!("Data: {}", packet.data);
        println!("----------");
    }
}
