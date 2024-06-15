use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::error::Error;
use reqwest::blocking::Client;
use serde_json::Value;

const WEATHER_FILE: &str = "/tmp/weather_info.txt";
const LOCATION_FILE: &str = "/tmp/location_info.txt";
const WEATHER_API_KEY: &str = "85a4e3c55b73909f42c6a23ec35b7147";

fn main() -> Result<(), Box<dyn Error>> {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let city = get_city_info(current_time)?;
    let weather_info = get_weather_info(&city, current_time)?;

    println!("{}", weather_info);
    Ok(())
}

fn get_city_info(current_time: u64) -> Result<String, Box<dyn Error>> {
    let last_update = get_last_update(LOCATION_FILE)?;

    if current_time - last_update >= 3600 {
        println!("Fetching new city information...");
        let client = Client::new();
        let res = client.get("https://ipinfo.io")
            .send()?
            .json::<Value>()?;

        let city = res["city"].as_str().unwrap_or("Unknown").to_string();
        write_to_file(LOCATION_FILE, &city)?;
        Ok(city)
    } else {
        println!("Reading city information from file...");
        let city = read_from_file(LOCATION_FILE)?;
        Ok(city)
    }
}

fn get_weather_info(city: &str, current_time: u64) -> Result<String, Box<dyn Error>> {
    let last_update = get_last_update(WEATHER_FILE)?;

    if current_time - last_update >= 3600 {
        println!("Fetching new weather information...");
        let client = Client::new();
        let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid={}", city, WEATHER_API_KEY);
        let res = client.get(&url)
            .send()?
            .json::<Value>()?;

        let temp = res["main"]["temp"].as_f64().unwrap_or(0.0);
        let description = res["weather"][0]["description"].as_str().unwrap_or("Unknown");

        let icon = match description {
            desc if desc.contains("sun") || desc.contains("clear") => "☀️",
            desc if desc.contains("cloud") => "🌤",
            desc if desc.contains("rain") => "☔️",
            desc if desc.contains("snow") => "❄️",
            _ => "☁️",
        };

        let weather_info = format!("{} {} {:.1}", icon, city, temp);
        write_to_file(WEATHER_FILE, &weather_info)?;
        Ok(weather_info)
    } else {
        println!("Reading weather information from file...");
        let weather_info = read_from_file(WEATHER_FILE)?;
        Ok(weather_info)
    }
}

fn get_last_update(file_path: &str) -> Result<u64, Box<dyn Error>> {
    match std::fs::metadata(file_path) {
        Ok(metadata) => Ok(metadata.modified()?.duration_since(UNIX_EPOCH)?.as_secs()),
        Err(_) => Ok(0),
    }
}

fn write_to_file(file_path: &str, data: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn read_from_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&contents)?;
    Ok(contents.trim().to_string())
}
