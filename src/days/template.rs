use crate::utils::DaySolver;

pub struct DayX;

impl DaySolver for DayX {
    fn part1(&self, input: &str) -> Option<String> {
        None
    }

    fn part2(&self, input: &str) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            TODO
        "};
        let solver = DayX {};
        assert_eq!(solver.part1(input).unwrap(), "TOOD");
        assert_eq!(solver.part2(input).unwrap(), "TODO");
    }
}
