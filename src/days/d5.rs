use std::collections::HashSet;

use petgraph::Graph;

use crate::utils::DaySolver;

pub struct Day5;

impl DaySolver for Day5 {
    fn part1(&self, input: &str) -> Option<String> {
        let input = input.replace("\r", "");
        let mut input = input.split("\n\n");

        let rules = input.next().unwrap();
        let updates = input.next().unwrap();

        let rules: Vec<(u32, u32)> = rules
            .lines()
            .map(|r| {
                let mut r = r.split("|");
                let a: u32 = r.next().unwrap().parse().unwrap();
                let b: u32 = r.next().unwrap().parse().unwrap();
                (a, b)
            })
            .collect();

        let graph = Graph::<usize, ()>::from_edges(rules.iter());

        let mut sum = 0;

        updates.lines().for_each(|u| {
            let vals: Vec<u32> = u.split(",").map(|m| m.parse().unwrap()).collect();
            let mut u = u.split(",");
            let mut prev: u32 = u.next().unwrap().parse().unwrap();
            let mut ok = true;
            u.for_each(|rr| {
                let rr: u32 = rr.parse().unwrap();
                let ii = graph.contains_edge(prev.into(), rr.into());
                if !ii {
                    ok = false;
                    return;
                }
                prev = rr;
            });
            if !ok {
                return;
            }
            sum += vals[vals.len() / 2];
        });

        Some(sum.to_string())
    }

    fn part2(&self, input: &str) -> Option<String> {
        let input = input.replace("\r", "");
        let mut input = input.split("\n\n");

        let rules = input.next().unwrap();
        let updates = input.next().unwrap();

        let rules: Vec<(u32, u32)> = rules
            .lines()
            .map(|r| {
                let mut r = r.split("|");
                let a: u32 = r.next().unwrap().parse().unwrap();
                let b: u32 = r.next().unwrap().parse().unwrap();
                (a, b)
            })
            .collect();

        let graph = Graph::<usize, ()>::from_edges(rules.iter());

        let mut sum = 0;

        updates.lines().for_each(|u| {
            let vals: Vec<u32> = u.split(",").map(|m| m.parse().unwrap()).collect();
            let mut u = u.split(",");
            let mut prev: u32 = u.next().unwrap().parse().unwrap();
            let mut ok = true;
            u.for_each(|rr| {
                let rr: u32 = rr.parse().unwrap();
                let ii = graph.contains_edge(prev.into(), rr.into());
                if !ii {
                    ok = false;
                    return;
                }
                prev = rr;
            });
            if ok {
                return;
            }

            // fix broken
            let mut correct = vec![];
            let mut unknown = HashSet::new();
            for i in vals.iter() {
                unknown.insert(*i);
            }

            while !unknown.is_empty() {
                let mut lala = 0;
                'a: for cur in unknown.iter() {
                    for i in unknown.iter() {
                        if cur == i {
                            continue;
                        }
                        if !graph.contains_edge((*cur).into(), (*i).into()) {
                            continue 'a;
                        }
                    }
                    correct.push(*cur);
                    lala = *cur;
                    break;
                }
                unknown.remove(&lala);
            }

            sum += correct[vals.len() / 2];
        });

        Some(sum.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
        "};
        let solver = Day5 {};
        assert_eq!(solver.part1(input).unwrap(), "143");
        assert_eq!(solver.part2(input).unwrap(), "123");
    }
}
