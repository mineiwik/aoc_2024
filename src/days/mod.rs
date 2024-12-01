use crate::utils::DaySolver;

mod d1;
mod d2;

pub fn get_solver(day: u8) -> Option<Box<dyn DaySolver>> {
    match day {
        1 => Some(Box::new(d1::Day1)),
        2 => Some(Box::new(d2::Day2)),
        _ => None,
    }
}
