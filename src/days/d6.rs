use std::collections::HashSet;

use crate::utils::DaySolver;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn rot_clockwise(&self) -> Self {
        match self {
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::North => Self::East,
        }
    }

    fn move_in_direction(&self, pos: &(isize, isize)) -> (isize, isize) {
        match self {
            Direction::East => (pos.0 + 1, pos.1),
            Direction::South => (pos.0, pos.1 + 1),
            Direction::West => (pos.0 - 1, pos.1),
            Direction::North => (pos.0, pos.1 - 1),
        }
    }
}

struct Map {
    obstacles: HashSet<(isize, isize)>,
    width: isize,
    height: isize,
}

impl Map {
    fn has_left_map(&self, pos: &(isize, isize)) -> bool {
        if pos.0 < 0 || pos.0 >= self.width {
            return true;
        }
        if pos.1 < 0 || pos.1 >= self.height {
            return true;
        }
        false
    }
}

struct Guard {
    pos: (isize, isize),
    dir: Direction,
    visited: HashSet<(isize, isize)>,
}

impl Map {
    fn has_obstacle_along(
        &self,
        mut pos: (isize, isize),
        dir: Direction,
    ) -> Option<(isize, isize)> {
        let mut cur_pos = dir.move_in_direction(&pos);
        while !self.has_left_map(&cur_pos) {
            if self.obstacles.contains(&cur_pos) {
                return Some(pos);
            }
            pos = cur_pos;
            cur_pos = dir.move_in_direction(&cur_pos);
        }
        None
    }
}

impl Guard {
    fn make_move(&mut self, map: &Map) {
        let new_pos = self.get_next_pos();
        if map.obstacles.contains(&new_pos) {
            self.turn_right();
            return;
        }
        self.pos = new_pos;
        if !self.has_left_map(map) {
            self.visited.insert(self.pos);
        }
    }

    fn turn_right(&mut self) {
        self.dir = self.dir.rot_clockwise();
    }

    fn has_left_map(&self, map: &Map) -> bool {
        if self.pos.0 < 0 || self.pos.0 >= map.width {
            return true;
        }
        if self.pos.1 < 0 || self.pos.1 >= map.height {
            return true;
        }
        false
    }

    fn get_next_pos(&self) -> (isize, isize) {
        self.dir.move_in_direction(&self.pos)
    }

    fn is_viable_obstacle(&self, map: &mut Map) -> bool {
        let new_pos = self.get_next_pos();
        if self.visited.contains(&new_pos) {
            return false;
        }
        if map.has_left_map(&new_pos) {
            return false;
        }
        if map.obstacles.contains(&new_pos) {
            return false;
        }
        map.obstacles.insert(new_pos);
        let mut visited = HashSet::new();
        let mut cur_pos = self.pos;
        let mut cur_dir = self.dir.rot_clockwise();
        while let Some(new_pos) = map.has_obstacle_along(cur_pos, cur_dir) {
            if visited.contains(&(cur_dir, cur_pos)) {
                map.obstacles.remove(&self.get_next_pos());
                return true;
            } else {
                visited.insert((cur_dir, cur_pos));
            }
            cur_pos = new_pos;
            cur_dir = cur_dir.rot_clockwise();
        }
        map.obstacles.remove(&self.get_next_pos());
        false
    }
}

pub struct Day6;

impl DaySolver for Day6 {
    fn part1(&self, input: &str) -> Option<String> {
        let (mut guard, map) = parse(input);

        while !guard.has_left_map(&map) {
            guard.make_move(&map);
        }
        Some(guard.visited.len().to_string())
    }

    fn part2(&self, input: &str) -> Option<String> {
        let (mut guard, mut map) = parse(input);
        let mut viable_ob = HashSet::new();

        while !guard.has_left_map(&map) {
            if guard.is_viable_obstacle(&mut map) {
                viable_ob.insert(guard.get_next_pos());
            }
            guard.make_move(&map);
        }

        Some(viable_ob.len().to_string())
    }
}

fn parse(input: &str) -> (Guard, Map) {
    let width = input.lines().next().unwrap().chars().count() as isize;
    let height = input.lines().count() as isize;
    let mut obstacles = HashSet::new();
    let mut guard_pos = (0, 0);
    let mut guard_dir = Direction::East;
    input
        .replace("\r", "")
        .replace("\n", "")
        .chars()
        .enumerate()
        .for_each(|(i, c)| {
            let current_pos = (i as isize % width, i as isize / width);
            match c {
                '#' => {
                    obstacles.insert(current_pos);
                }
                '>' => {
                    guard_pos = current_pos;
                    guard_dir = Direction::East;
                }
                'v' => {
                    guard_pos = current_pos;
                    guard_dir = Direction::South;
                }
                '<' => {
                    guard_pos = current_pos;
                    guard_dir = Direction::West;
                }
                '^' => {
                    guard_pos = current_pos;
                    guard_dir = Direction::North;
                }
                _ => {}
            }
        });
    let map = Map {
        obstacles,
        width,
        height,
    };
    let guard = Guard {
        pos: guard_pos,
        dir: guard_dir,
        visited: HashSet::new(),
    };
    (guard, map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
        "};
        let solver = Day6 {};
        assert_eq!(solver.part1(input).unwrap(), "41");
        assert_eq!(solver.part2(input).unwrap(), "6");
    }
}
