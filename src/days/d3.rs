use regex::Regex;

use crate::utils::DaySolver;

pub struct Day3;

const MUL_REG: &str = r"mul\((\d+),(\d+)\)";
const INSTR_REG: &str = r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))";

impl DaySolver for Day3 {
    fn part1(&self, input: &str) -> Option<String> {
        let mul_re = Regex::new(MUL_REG).unwrap();
        let mut sum = 0;
        for (_, [op1, op2]) in mul_re.captures_iter(input).map(|c| c.extract()) {
            sum += op1.parse::<isize>().unwrap() * op2.parse::<isize>().unwrap();
        }
        Some(sum.to_string())
    }

    fn part2(&self, input: &str) -> Option<String> {
        let mul_re = Regex::new(INSTR_REG).unwrap();
        let mut sum = 0;
        let mut mul_enabled = true;
        for c in mul_re.captures_iter(input) {
            let mut c = c.iter().flatten();
            let instr = c.next()?.as_str();
            match instr {
                "don't()" => mul_enabled = false,
                "do()" => mul_enabled = true,
                _ if mul_enabled => {
                    c.next();
                    let op1: isize = c.next()?.as_str().parse().unwrap();
                    let op2: isize = c.next()?.as_str().parse().unwrap();
                    sum += op1 * op2;
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
