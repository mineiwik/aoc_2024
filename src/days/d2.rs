use crate::utils::DaySolver;

pub struct Day2;

impl DaySolver for Day2 {
    fn part1(&self, _input: &str) -> Option<String> {
        None
    }

    fn part2(&self, _input: &str) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let _input = indoc! {"
            TODO
        "};
        let _solver = Day2 {};
        //assert_eq!(solver.part1(input).unwrap(), "TODO");
        //assert_eq!(solver.part2(input).unwrap(), "TODO");
    }
}
