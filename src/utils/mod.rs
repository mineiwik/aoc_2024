mod day;
mod fetch;

use std::{
    env,
    error::Error,
    fmt,
    fs::File,
    io::{self, Read, Write},
    time,
};

use colored::{Colorize, CustomColor};
pub use day::DaySolver;
use fetch::fetch_input;

use crate::{days::get_solver, AOC_YEAR};

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
    let input = fetch_input(AOC_YEAR, day).await?;
    let mut file = File::create(&filename)?;
    file.write_all(input.as_bytes())?;
    Ok(input)
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
    let timer = time::Instant::now();
    let p2 = solver.part2(&input);
    let p2_time = timer.elapsed().as_micros();

    display_part_result(1, p1, p1_time);
    display_part_result(2, p2, p2_time);
    println!();
    Ok(())
}

pub fn get_day() -> Result<u8, Box<dyn Error>> {
    print!("Enter day: ");
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
        "*".bright_yellow(),
        part,
        part_res.bright_yellow(),
        (time as f64) / 1000.0
    );
}

pub fn display_banner(message: &str, x_padding: usize) {
    let x_padding = " ".repeat(x_padding).on_custom_color(AOC_BLUE);
    let y_border = "*"
        .repeat(message.len() + 4)
        .bright_yellow()
        .on_custom_color(AOC_BLUE);

    // Clear terminal
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!(
        "{x_padding}{y_border}{x_padding}",
        x_padding = x_padding,
        y_border = y_border
    );
    println!(
        "{x_padding}{left_border}{message}{right_border}{x_padding}",
        x_padding = x_padding,
        left_border = "* ".bright_yellow().on_custom_color(AOC_BLUE),
        right_border = " *".bright_yellow().on_custom_color(AOC_BLUE),
        message = message.bright_green().on_custom_color(AOC_BLUE),
    );
    println!(
        "{x_padding}{y_border}{x_padding}",
        x_padding = x_padding,
        y_border = y_border
    );
    println!();
}
