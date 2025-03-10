use clap::Parser;
use reqwest;
use serde::Deserialize;
use serde_json;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct GeoIp {
    lat: f64,
    lon: f64,
    city: String,
    country: String,
}

#[derive(Deserialize, Debug)]
struct Meteo {
    current_units: CurrentUnits,
    current: Current,
}

#[derive(Deserialize, Debug)]
struct CurrentUnits {
    temperature_2m: String,
    relative_humidity_2m: String,
    precipitation: String,
    rain: String,
    showers: String,
    snowfall: String,
}

#[derive(Deserialize, Debug)]
struct Current {
    time: String,
    temperature_2m: f64,
    relative_humidity_2m: f64,
    apparent_temperature: f64,
    is_day: i32,
    precipitation: f64,
    rain: f64,
    showers: f64,
    snowfall: f64,
}

#[derive(Parser, Debug)]
#[command(author = "me", version = "0.1.0", about = "A CLI tool to display the current weather information", long_about = None)]
struct Args {
    /// Display only the temperature.
    #[clap(long, action)]
    temperature: bool,

    /// Display only the relative humidity.
    #[clap(long, action)]
    humidity: bool,

    /// Display only the apparent temperature.
    #[clap(long, action)]
    apparent_temperature: bool,

    /// Show `0` for night and `1` for day.
    #[clap(long, action)]
    nightorday: bool,

    /// Display only the precipitation.
    #[clap(long, action)]
    precipitation: bool,

    /// Display only the rain.
    #[clap(long, action)]
    rain: bool,

    /// Display only the showers.
    #[clap(long, action)]
    showers: bool,

    /// Display only the snowfall.
    #[clap(long, action)]
    snowfall: bool,

    /// Display only the IP.
    #[clap(long, action)]
    ip: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Get IP address
    let ip_address = get_ip().await?;

    // Get location from GeoIP
    let geoip = get_geoip(&ip_address).await?;

    // Get weather from Open-Meteo
    let meteo = get_meteo(geoip.lat, geoip.lon).await?;

    if args.temperature {
        println!("{}", meteo.current.temperature_2m);
    } else if args.humidity {
        println!("{}", meteo.current.relative_humidity_2m);
    } else if args.apparent_temperature {
        println!("{}", meteo.current.apparent_temperature);
    } else if args.nightorday {
        println!("{}", meteo.current.is_day);
    } else if args.precipitation {
        println!("{}", meteo.current.precipitation);
    } else if args.rain {
        println!("{}", meteo.current.rain);
    } else if args.showers {
        println!("{}", meteo.current.showers);
    } else if args.snowfall {
        println!("{}", meteo.current.snowfall);
    } else if args.ip {
        println!("{}", ip_address);
    } else {
        println!("Location: {}, {}", geoip.city, geoip.country);
        println!(
            "Time: {} ({})",
            meteo.current.time,
            if meteo.current.is_day == 0 {
                "Night"
            } else {
                "Day"
            }
        );
        println!(
            "Temperature: {} {}",
            meteo.current.temperature_2m, meteo.current_units.temperature_2m
        );
        println!(
            "Relative Humidity: {} {}",
            meteo.current.relative_humidity_2m, meteo.current_units.relative_humidity_2m
        );
        println!(
            "Apparent Temperature: {}",
            meteo.current.apparent_temperature
        );
        println!(
            "Precipitation: {} {}",
            meteo.current.precipitation, meteo.current_units.precipitation
        );
        println!("Rain: {} {}", meteo.current.rain, meteo.current_units.rain);
        println!(
            "Showers: {} {}",
            meteo.current.showers, meteo.current_units.showers
        );
        println!(
            "Snowfall: {} {}",
            meteo.current.snowfall, meteo.current_units.snowfall
        );
    }

    Ok(())
}

async fn get_ip() -> Result<String, Box<dyn Error>> {
    let resp = reqwest::get("https://api.ipify.org?format=json")
        .await?
        .text()
        .await?;

    let json: serde_json::Value = serde_json::from_str(&resp)?;
    let ip = json["ip"].as_str().unwrap().to_string();
    Ok(ip)
}

async fn get_geoip(ip: &str) -> Result<GeoIp, Box<dyn Error>> {
    let url = format!("http://ip-api.com/json/{}", ip);
    let resp = reqwest::get(url).await?.text().await?;

    let geoip: GeoIp = serde_json::from_str(&resp)?;
    Ok(geoip)
}

async fn get_meteo(lat: f64, lon: f64) -> Result<Meteo, Box<dyn Error>> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,apparent_temperature,is_day,precipitation,rain,showers,snowfall",
        lat, lon
    );
    let resp = reqwest::get(url).await?.text().await?;

    let meteo: Meteo = serde_json::from_str(&resp)?;
    Ok(meteo)
}
