use std::collections::HashMap;

use crate::utils::DaySolver;

pub struct Day1;

impl DaySolver for Day1 {
    fn part1(&self, input: &str) -> Option<String> {
        let (l1, l2) = parse_part1(input);
        let mut sum = 0;
        for (n1, n2) in l1.iter().zip(l2.iter()) {
            sum += (n1 - n2).abs();
        }
        Some(sum.to_string())
    }

    fn part2(&self, input: &str) -> Option<String> {
        let (l1, l2) = parse_part2(input);
        let mut sum = 0;
        for (n1, c1) in l1 {
            let Some(c2) = l2.get(&n1) else {
                continue;
            };
            sum += c1 * (n1 * c2);
        }

        Some(sum.to_string())
    }
}

fn parse_part1(input: &str) -> (Vec<isize>, Vec<isize>) {
    let mut l1 = vec![];
    let mut l2 = vec![];
    input.lines().for_each(|l| {
        let mut l = l.split("   ");
        let n1 = l.next().unwrap().parse().unwrap();
        let n2 = l.next().unwrap().parse().unwrap();
        l1.push(n1);
        l2.push(n2);
    });
    l1.sort();
    l2.sort();
    (l1, l2)
}

fn parse_part2(input: &str) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
    let mut l1 = HashMap::new();
    let mut l2 = HashMap::new();
    input.lines().for_each(|l| {
        let mut l = l.split("   ");
        let n1 = l.next().unwrap().parse().unwrap();
        let n2 = l.next().unwrap().parse().unwrap();
        l1.entry(n1)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
        l2.entry(n2)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    });
    (l1, l2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "};
        let solver = Day1 {};
        assert_eq!(solver.part1(input).unwrap(), "11");
        assert_eq!(solver.part2(input).unwrap(), "31");
    }
}
