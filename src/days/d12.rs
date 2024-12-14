use std::collections::{HashMap, HashSet};

use crate::utils::DaySolver;

enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    fn get_main_directions() -> Vec<Self> {
        vec![Self::Up, Self::Right, Self::Down, Self::Left]
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn get_pos_in_dir(&self, dir: Direction) -> Self {
        match dir {
            Direction::Down => Self::new(self.x, self.y + 1),
            Direction::Up => Self::new(self.x, self.y - 1),
            Direction::Left => Self::new(self.x - 1, self.y),
            Direction::Right => Self::new(self.x + 1, self.y),
            Direction::UpRight => Self::new(self.x + 1, self.y - 1),
            Direction::DownRight => Self::new(self.x + 1, self.y + 1),
            Direction::UpLeft => Self::new(self.x - 1, self.y - 1),
            Direction::DownLeft => Self::new(self.x - 1, self.y + 1),
        }
    }
}

struct Region {
    area: HashSet<Position>,
}

impl Region {
    fn contains(&self, pos: Position, dir: Direction) -> bool {
        self.area.contains(&pos.get_pos_in_dir(dir))
    }

    fn count_outer_corners(&self, pos: Position) -> usize {
        if !self.area.contains(&pos) {
            return 0;
        }
        let mut sum = 0;
        if !self.contains(pos, Direction::Right) && !self.contains(pos, Direction::Up) {
            sum += 1;
        }
        if !self.contains(pos, Direction::Right) && !self.contains(pos, Direction::Down) {
            sum += 1;
        }
        if !self.contains(pos, Direction::Left) && !self.contains(pos, Direction::Up) {
            sum += 1;
        }
        if !self.contains(pos, Direction::Left) && !self.contains(pos, Direction::Down) {
            sum += 1;
        }
        sum
    }

    fn count_inner_corners(&self, pos: Position) -> usize {
        if !self.area.contains(&pos) {
            return 0;
        }
        let mut sum = 0;
        if self.contains(pos, Direction::Right)
            && self.contains(pos, Direction::Up)
            && !self.contains(pos, Direction::UpRight)
        {
            sum += 1;
        }
        if self.contains(pos, Direction::Right)
            && self.contains(pos, Direction::Down)
            && !self.contains(pos, Direction::DownRight)
        {
            sum += 1;
        }
        if self.contains(pos, Direction::Left)
            && self.contains(pos, Direction::Up)
            && !self.contains(pos, Direction::UpLeft)
        {
            sum += 1;
        }
        if self.contains(pos, Direction::Left)
            && self.contains(pos, Direction::Down)
            && !self.contains(pos, Direction::DownLeft)
        {
            sum += 1;
        }
        sum
    }

    fn calc_sides(&mut self) -> usize {
        let mut sides = 0;
        for pos in self.area.iter() {
            sides += self.count_outer_corners(*pos);
            sides += self.count_inner_corners(*pos);
        }
        sides
    }

    fn calc_edges(&mut self) -> usize {
        let mut sum = 0;
        for pos in self.area.iter() {
            if !self.contains(*pos, Direction::Right) {
                sum += 1;
            }
            if !self.contains(*pos, Direction::Left) {
                sum += 1;
            }
            if !self.contains(*pos, Direction::Up) {
                sum += 1;
            }
            if !self.contains(*pos, Direction::Down) {
                sum += 1;
            }
        }
        sum
    }
}

pub struct Day12;

impl DaySolver for Day12 {
    fn part1(&self, input: &str) -> Option<String> {
        let mut regions = parse(input);

        let mut sum = 0;
        for r in &mut regions {
            let edges = r.calc_edges();
            sum += edges * r.area.len();
        }
        Some(sum.to_string())
    }

    fn part2(&self, input: &str) -> Option<String> {
        let mut regions = parse(input);
        let mut sum = 0;
        for r in &mut regions {
            let sides = r.calc_sides();
            sum += sides * r.area.len();
        }
        Some(sum.to_string())
    }
}

fn parse(input: &str) -> Vec<Region> {
    let mut positions = HashMap::new();
    for (y, l) in input.lines().enumerate() {
        for (x, a) in l.chars().enumerate() {
            positions.insert(Position::new(x as isize, y as isize), a);
        }
    }
    let mut regions = vec![];
    while !positions.is_empty() {
        let pos = *positions.keys().next().unwrap();
        let region_id = *positions.get(&pos).unwrap();
        regions.push(explore_region(&mut positions, region_id, pos));
    }
    regions
}

fn explore_region(
    positions: &mut HashMap<Position, char>,
    region_id: char,
    pos: Position,
) -> Region {
    let mut area = HashSet::new();
    let mut to_explore = vec![pos];
    while let Some(cur_pos) = to_explore.pop() {
        area.insert(cur_pos);
        positions.remove(&cur_pos);
        for dir in Direction::get_main_directions() {
            let new_pos = cur_pos.get_pos_in_dir(dir);
            if let Some(c) = positions.get(&new_pos) {
                if c.eq(&region_id) {
                    to_explore.push(new_pos);
                }
            }
        }
    }
    Region { area }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE
        "};
        let solver = Day12 {};
        assert_eq!(solver.part1(input).unwrap(), "1930");
        assert_eq!(solver.part2(input).unwrap(), "1206");
    }
}
