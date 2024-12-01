//! fetch.rs
//!
//! Allows to automatically fetch AoC puzzle inputs
//!

use std::{env, error::Error};

const AOC_FQDN: &str = "https://adventofcode.com";

/// Fetches the puzzle input corresponding to the given year and day parameters directly from the AoC Website
///
/// # Arguments
/// * `year` - The year in the `YYYY` format
/// * `day` - The day
pub async fn fetch_input(year: u16, day: u8) -> Result<String, Box<dyn Error>> {
    let url = format!("{AOC_FQDN}/{year}/day/{day}/input");

    let cookie = env::var("AOC_SESSION")?;

    let client = reqwest::Client::new();
    let res = client.get(url).header("Cookie", cookie).send().await?;

    Ok(res.text().await?)
}
