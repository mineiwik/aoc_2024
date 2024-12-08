use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::utils::DaySolver;

pub struct Day8;

type Antennas = HashMap<char, Vec<(isize, isize)>>;

impl DaySolver for Day8 {
    fn part1(&self, input: &str) -> Option<String> {
        let (width, height, antennas) = parse(input);
        let mut antinodes = HashSet::new();

        for (_, positions) in antennas {
            for p in positions.iter().permutations(2) {
                let a = p[0];
                let b = p[1];
                let n = get_antinodes(*a, *b, width, height, false);
                antinodes.extend(n);
            }
        }

        Some(antinodes.len().to_string())
    }

    fn part2(&self, input: &str) -> Option<String> {
        let (width, height, antennas) = parse(input);
        let mut antinodes = HashSet::new();

        for (_, positions) in antennas {
            for p in positions.iter().permutations(2) {
                let a = p[0];
                let b = p[1];
                let n = get_antinodes(*a, *b, width, height, true);
                antinodes.extend(n);
            }
        }

        Some(antinodes.len().to_string())
    }
}

fn get_antinodes(
    mut a: (isize, isize),
    mut b: (isize, isize),
    width: isize,
    height: isize,
    part2: bool,
) -> Vec<(isize, isize)> {
    let mut antinodes = vec![];
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    if !part2 {
        a = (a.0 + dx, a.1 + dy);
        b = (b.0 - dx, b.1 - dy);
    }
    while a.0 >= 0 && a.0 < width && a.1 >= 0 && a.1 < height {
        antinodes.push(a);
        if !part2 {
            break;
        }
        a = (a.0 + dx, a.1 + dy);
    }
    while b.0 >= 0 && b.0 < width && b.1 >= 0 && b.1 < height {
        antinodes.push(b);
        if !part2 {
            break;
        }
        b = (b.0 - dx, b.1 - dy);
    }
    antinodes
}

fn parse(input: &str) -> (isize, isize, Antennas) {
    let width = input.lines().next().unwrap().chars().count() as isize;
    let height = input.lines().count() as isize;
    let mut antennas = HashMap::new();
    input
        .replace("\r", "")
        .replace("\n", "")
        .chars()
        .enumerate()
        .for_each(|(i, c)| {
            let current_pos = (i as isize % width, i as isize / width);
            match c {
                '.' => {}
                c => {
                    antennas
                        .entry(c)
                        .and_modify(|e: &mut Vec<(isize, isize)>| e.push(current_pos))
                        .or_insert(vec![current_pos]);
                }
            }
        });
    (width, height, antennas)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            ............
            ........0...
            .....0......
            .......0....
            ....0.......
            ......A.....
            ............
            ............
            ........A...
            .........A..
            ............
            ............
        "};
        let solver = Day8 {};
        assert_eq!(solver.part1(input).unwrap(), "14");
        assert_eq!(solver.part2(input).unwrap(), "34");
    }
}
