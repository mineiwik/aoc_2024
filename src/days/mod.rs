use crate::utils::DaySolver;

mod d1;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;

pub fn get_solver(day: u8) -> Option<Box<dyn DaySolver>> {
    match day {
        1 => Some(Box::new(d1::Day1)),
        2 => Some(Box::new(d2::Day2)),
        3 => Some(Box::new(d3::Day3)),
        4 => Some(Box::new(d4::Day4)),
        5 => Some(Box::new(d5::Day5)),
        6 => Some(Box::new(d6::Day6)),
        7 => Some(Box::new(d7::Day7)),
        8 => Some(Box::new(d8::Day8)),
        9 => Some(Box::new(d9::Day9)),
        10 => Some(Box::new(d10::Day10)),
        11 => Some(Box::new(d11::Day11)),
        12 => Some(Box::new(d12::Day12)),
        13 => Some(Box::new(d13::Day13)),
        14 => Some(Box::new(d14::Day14 {
            width: 101,
            height: 103,
        })),
        _ => None,
    }
}
