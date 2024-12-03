use regex::Regex;

use crate::utils::DaySolver;

pub struct Day3;

impl DaySolver for Day3 {
    fn part1(&self, input: &str) -> Option<String> {
        let mul_re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))").unwrap();
        let op_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let mut sum = 0;
        for (_, [mul]) in mul_re.captures_iter(input).map(|c| c.extract()) {
            let c = op_re.captures(mul).unwrap();
            sum += c[1].parse::<isize>().unwrap() * c[2].parse::<isize>().unwrap();
        }
        Some(sum.to_string())
    }

    fn part2(&self, input: &str) -> Option<String> {
        let mul_re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap();
        let op_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        let mut sum = 0;
        let mut mul_enabled = true;
        for (_, [mul]) in mul_re.captures_iter(input).map(|c| c.extract()) {
            match mul {
                "don't()" => mul_enabled = false,
                "do()" => mul_enabled = true,
                _ if mul_enabled => {
                    let c = op_re.captures(mul).unwrap();
                    sum += c[1].parse::<isize>().unwrap() * c[2].parse::<isize>().unwrap();
                }
                _ => {}
            }
        }
        Some(sum.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input1 = indoc! {"
            xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
        "};
        let input2 = indoc! {"
            xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
        "};
        let solver = Day3 {};
        assert_eq!(solver.part1(input1).unwrap(), "161");
        assert_eq!(solver.part2(input2).unwrap(), "48");
    }
}
