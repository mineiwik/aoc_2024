mod day;
mod fetch;
#[cfg(test)]
mod mock_time;

use std::{
    env,
    error::Error,
    fmt,
    fs::File,
    io::{self, Read, Write},
    time,
};

#[cfg(not(test))]
use chrono::Utc;
#[cfg(test)]
use mock_time::Utc;

use chrono::{Datelike, Timelike};
use colored::{Colorize, CustomColor};
pub use day::DaySolver;
use fetch::fetch_input;

use crate::{days::get_solver, AOC_YEAR};

const AOC_MONTH: u32 = 12;
const AOC_UTC_HOUR: u32 = 5;

const AOC_GRAY: CustomColor = CustomColor {
    r: 105,
    g: 105,
    b: 105,
};
const AOC_BLUE: CustomColor = CustomColor {
    r: 15,
    g: 15,
    b: 35,
};
const AOC_YELLOW: CustomColor = CustomColor {
    r: 255,
    g: 255,
    b: 72,
};
const AOC_GREEN: CustomColor = CustomColor { r: 0, g: 176, b: 0 };

#[derive(Debug, Clone)]
enum AoCError {
    InvalidDay,
    NoInput,
}

impl fmt::Display for AoCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDay => write!(f, "Invalid Day"),
            Self::NoInput => write!(f, "Did not find any input for the selected day"),
        }
    }
}

impl Error for AoCError {}

async fn get_input(day: u8) -> Result<String, Box<dyn Error>> {
    let filename = format!("inputs/day{}.txt", day);
    if let io::Result::Ok(mut file) = File::open(&filename) {
        let mut input = String::new();
        let _ = file.read_to_string(&mut input);
        return Ok(input);
    };

    if env::var("AOC_SESSION").is_err() {
        println!("Fetching input for day {day} from AoC Website is not possible, as the AOC_SESSION env variable is not present. Either place it in `.env` or enable it temporarily in your shell session.");
        println!(
            "You can also grab the input yourself and place it under `inputs/day{}.txt`",
            day
        );
        return Err(AoCError::NoInput.into());
    }

    if !is_puzzle_available(day) {
        println!("Unable to fetch unpublished puzzle. Please come back later!");
        return Err(AoCError::NoInput.into());
    }

    let input = fetch_input(AOC_YEAR, day).await?;
    let mut file = File::create(&filename)?;
    file.write_all(input.as_bytes())?;
    Ok(input)
}

fn is_puzzle_available(day: u8) -> bool {
    let now = Utc::now();

    if now.year() < AOC_YEAR.into() {
        return false;
    }

    if now.year() > AOC_YEAR.into() {
        return true;
    }

    if now.month() < AOC_MONTH {
        return false;
    }

    if now.day() < day.into() {
        return false;
    }

    if now.day() > day.into() {
        return true;
    }

    if now.hour() < AOC_UTC_HOUR {
        return false;
    }

    true
}

pub async fn solve_day(day: u8) -> Result<(), Box<dyn Error>> {
    let Some(solver) = get_solver(day) else {
        println!("({}) Day not solved yet!", "*".custom_color(AOC_GRAY));
        println!();
        return Ok(());
    };

    let input = get_input(day).await?;

    let timer = time::Instant::now();
    let p1 = solver.part1(&input);
    let p1_time = timer.elapsed().as_micros();

    display_part_result(1, p1, p1_time);

    let timer = time::Instant::now();
    let p2 = solver.part2(&input);
    let p2_time = timer.elapsed().as_micros();

    display_part_result(2, p2, p2_time);

    println!();
    Ok(())
}

pub fn get_day() -> Result<u8, Box<dyn Error>> {
    print!("Enter day (leave empty to solve all days): ");
    io::stdout().flush()?;
    let mut day = String::new();
    io::stdin().read_line(&mut day).unwrap();
    day = day.replace("\n", "").replace("\r", "");
    let day = day.parse()?;
    if day > 25 {
        return Err(AoCError::InvalidDay.into());
    }
    Ok(day)
}

fn display_part_result(part: u8, part_res: Option<String>, time: u128) {
    let Some(part_res) = part_res else {
        println!(
            "({}) Part {}: Not solved yet",
            "*".custom_color(AOC_GRAY),
            part,
        );
        return;
    };

    println!(
        "({}) Part {}: {} (took {} ms)",
        "*".custom_color(AOC_YELLOW),
        part,
        part_res.custom_color(AOC_YELLOW),
        (time as f64) / 1000.0
    );
}

pub fn display_banner(message: &str, x_padding: usize, clear: bool) {
    let x_padding = " ".repeat(x_padding).on_custom_color(AOC_BLUE);
    let y_border = "*"
        .repeat(message.len() + 4)
        .custom_color(AOC_YELLOW)
        .on_custom_color(AOC_BLUE);

    // Clear terminal
    if clear {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
    println!(
        "{x_padding}{y_border}{x_padding}",
        x_padding = x_padding,
        y_border = y_border
    );
    println!(
        "{x_padding}{left_border}{message}{right_border}{x_padding}",
        x_padding = x_padding,
        left_border = "* ".custom_color(AOC_YELLOW).on_custom_color(AOC_BLUE),
        right_border = " *".custom_color(AOC_YELLOW).on_custom_color(AOC_BLUE),
        message = message.custom_color(AOC_GREEN).on_custom_color(AOC_BLUE),
    );
    println!(
        "{x_padding}{y_border}{x_padding}",
        x_padding = x_padding,
        y_border = y_border
    );
    println!();
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use mock_time::set_timestamp;

    use super::*;

    #[test]
    fn is_puzzle_available_test_day1_unpublished_1_year_to_go() {
        let dt = chrono::Utc
            .with_ymd_and_hms((AOC_YEAR - 1).into(), 12, 1, 5, 0, 0)
            .unwrap();
        set_timestamp(dt.timestamp());

        let res = is_puzzle_available(1);

        assert_eq!(res, false);
    }

    #[test]
    fn is_puzzle_available_test_day1_unpublished_1_month_to_go() {
        let dt = chrono::Utc
            .with_ymd_and_hms(AOC_YEAR.into(), 11, 1, 5, 0, 0)
            .unwrap();
        set_timestamp(dt.timestamp());

        let res = is_puzzle_available(1);

        assert_eq!(res, false);
    }

    #[test]
    fn is_puzzle_available_test_day1_unpublished_1_day_to_go() {
        let dt = chrono::Utc
            .with_ymd_and_hms(AOC_YEAR.into(), 11, 30, 5, 0, 0)
            .unwrap();
        set_timestamp(dt.timestamp());

        let res = is_puzzle_available(1);

        assert_eq!(res, false);
    }

    #[test]
    fn is_puzzle_available_test_day1_unpublished_1_hour_to_go() {
        let dt = chrono::Utc
            .with_ymd_and_hms(AOC_YEAR.into(), 12, 1, 4, 0, 0)
            .unwrap();
        set_timestamp(dt.timestamp());

        let res = is_puzzle_available(1);

        assert_eq!(res, false);
    }

    #[test]
    fn is_puzzle_available_test_day1_unpublished_1_sec_to_go() {
        let dt = chrono::Utc
            .with_ymd_and_hms(AOC_YEAR.into(), 12, 1, 4, 59, 59)
            .unwrap();
        set_timestamp(dt.timestamp());

        let res = is_puzzle_available(1);

        assert_eq!(res, false);
    }

    #[test]
    fn is_puzzle_available_test_day14_unpublished_1_sec_to_go() {
        let dt = chrono::Utc
            .with_ymd_and_hms(AOC_YEAR.into(), 12, 14, 4, 59, 59)
            .unwrap();
        set_timestamp(dt.timestamp());

        let res = is_puzzle_available(14);

        assert_eq!(res, false);
    }

    #[test]
    fn is_puzzle_available_test_day1_published_now() {
        let dt = chrono::Utc
            .with_ymd_and_hms(AOC_YEAR.into(), 12, 1, 5, 0, 0)
            .unwrap();
        set_timestamp(dt.timestamp());

        let res = is_puzzle_available(1);

        assert_eq!(res, true);
    }

    #[test]
    fn is_puzzle_available_test_day14_published_now() {
        let dt = chrono::Utc
            .with_ymd_and_hms(AOC_YEAR.into(), 12, 14, 5, 0, 0)
            .unwrap();
        set_timestamp(dt.timestamp());

        let res = is_puzzle_available(14);

        assert_eq!(res, true);
    }

    #[test]
    fn is_puzzle_available_test_day1_published_1_sec_ago() {
        let dt = chrono::Utc
            .with_ymd_and_hms(AOC_YEAR.into(), 12, 1, 5, 0, 1)
            .unwrap();
        set_timestamp(dt.timestamp());

        let res = is_puzzle_available(1);

        assert_eq!(res, true);
    }
}
