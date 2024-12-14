use std::collections::HashSet;

use crate::utils::DaySolver;

pub struct Day10;

impl DaySolver for Day10 {
    fn part1(&self, input: &str) -> Option<String> {
        let map: Vec<Vec<u8>> = input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect();

        let height = map.len();
        let width = map[0].len();

        let mut sum = 0;
        for y in 0..height {
            for x in 0..width {
                let e = map[y][x];
                if e == 9 {
                    let mut heads = HashSet::new();
                    get_trail_heads(&map, x, y, 9, &mut heads);
                    sum += heads.len();
                }
            }
        }

        Some(sum.to_string())
    }

    fn part2(&self, input: &str) -> Option<String> {
        let map: Vec<Vec<u8>> = input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect();

        let height = map.len();
        let width = map[0].len();

        let mut sum = 0;
        for y in 0..height {
            for x in 0..width {
                let e = map[y][x];
                if e == 9 {
                    let mut visited = HashSet::new();
                    sum += get_trail_heads2(&map, x, y, 9, &mut visited);
                }
            }
        }

        Some(sum.to_string())
    }
}

fn get_trail_heads(
    map: &Vec<Vec<u8>>,
    x: usize,
    y: usize,
    expected: u8,
    heads: &mut HashSet<(usize, usize)>,
) {
    if map[y][x] != expected {
        return;
    }
    if expected == 0 {
        heads.insert((x, y));
        return;
    }
    if x + 1 < map[0].len() {
        get_trail_heads(map, x + 1, y, expected - 1, heads);
    }
    if y + 1 < map.len() {
        get_trail_heads(map, x, y + 1, expected - 1, heads);
    }
    if x > 0 {
        get_trail_heads(map, x - 1, y, expected - 1, heads);
    }
    if y > 0 {
        get_trail_heads(map, x, y - 1, expected - 1, heads);
    }
}

fn get_trail_heads2(
    map: &Vec<Vec<u8>>,
    x: usize,
    y: usize,
    expected: u8,
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    if map[y][x] != expected {
        return 0;
    }
    if expected == 0 {
        return 1;
    }
    visited.insert((x, y));
    let mut sum = 0;
    if x + 1 < map[0].len() {
        sum += get_trail_heads2(map, x + 1, y, expected - 1, &mut visited.clone());
    }
    if y + 1 < map.len() {
        sum += get_trail_heads2(map, x, y + 1, expected - 1, &mut visited.clone());
    }
    if x > 0 {
        sum += get_trail_heads2(map, x - 1, y, expected - 1, &mut visited.clone());
    }
    if y > 0 {
        sum += get_trail_heads2(map, x, y - 1, expected - 1, &mut visited.clone());
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732
        "};
        let solver = Day10 {};
        assert_eq!(solver.part1(input).unwrap(), "36");
        assert_eq!(solver.part2(input).unwrap(), "81");
    }
}
