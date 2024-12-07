use crate::utils::DaySolver;

pub struct Day7;

impl DaySolver for Day7 {
    fn part1(&self, input: &str) -> Option<String> {
        Some(
            parse(input)
                .iter()
                .fold(0, |mut sum, (res, operands)| {
                    if check(*res, operands[0], &operands[1..], false) {
                        sum += *res;
                    }
                    sum
                })
                .to_string(),
        )
    }

    fn part2(&self, input: &str) -> Option<String> {
        Some(
            parse(input)
                .iter()
                .fold(0, |mut sum, (res, operands)| {
                    if check(*res, operands[0], &operands[1..], true) {
                        sum += *res;
                    }
                    sum
                })
                .to_string(),
        )
    }
}

fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|l| {
            let mut l = l.split(": ");
            let res = l.next().unwrap().parse().unwrap();
            let operands = l
                .next()
                .unwrap()
                .split(" ")
                .map(|o| o.parse().unwrap())
                .collect();
            (res, operands)
        })
        .collect()
}

fn check(res: usize, cur: usize, operands: &[usize], part2: bool) -> bool {
    if operands.is_empty() {
        return res == cur;
    }

    let add = check(res, cur + operands.first().unwrap(), &operands[1..], part2);
    let mul = check(res, cur * operands.first().unwrap(), &operands[1..], part2);
    let concat = if part2 {
        let mut a = cur.to_string();
        a.push_str(&operands.first().unwrap().to_string());
        check(res, a.parse().unwrap(), &operands[1..], part2)
    } else {
        false
    };

    add || mul || concat
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
        "};
        let solver = Day7 {};
        assert_eq!(solver.part1(input).unwrap(), "3749");
        assert_eq!(solver.part2(input).unwrap(), "11387");
    }
}
