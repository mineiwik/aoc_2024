use crate::utils::DaySolver;

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;

pub fn get_solver(day: u8) -> Option<Box<dyn DaySolver>> {
    match day {
        1 => Some(Box::new(d1::Day1)),
        2 => Some(Box::new(d2::Day2)),
        3 => Some(Box::new(d3::Day3)),
        4 => Some(Box::new(d4::Day4)),
        5 => Some(Box::new(d5::Day5)),
        6 => Some(Box::new(d6::Day6)),
        7 => Some(Box::new(d7::Day7)),
        _ => None,
    }
}
