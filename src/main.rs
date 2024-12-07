mod days;
mod utils;

use std::error::Error;

use utils::*;

const AOC_YEAR: u16 = 2024;
const X_PADDING: usize = 1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    display_banner("Advent of Code 2024", X_PADDING);
    if dotenvy::dotenv().is_err() {
        println!("No .env file detected. AOC_SESSION variable may not be set and auto-fetching puzzle inputs is not possible!")
    }
    let day = get_day()?;
    display_banner(&format!("Advent of Code 2024 - Day {}", day), 1);

    solve_day(day).await
}
