use std::{collections::HashSet, fmt::Error, str::FromStr};

use crate::utils::DaySolver;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Self::Left),
            ">" => Ok(Self::Right),
            "^" => Ok(Self::Up),
            "v" => Ok(Self::Down),
            _ => Err(Error),
        }
    }
}

#[derive(PartialEq, PartialOrd, Hash, Clone, Copy, Eq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn pos_in_dir(&self, dir: Instruction) -> Self {
        match dir {
            Instruction::Down => Self::new(self.x, self.y + 1),
            Instruction::Up => Self::new(self.x, self.y - 1),
            Instruction::Left => Self::new(self.x - 1, self.y),
            Instruction::Right => Self::new(self.x + 1, self.y),
        }
    }
}

struct Warehouse {
    robot: Position,
    walls: HashSet<Position>,
    boxes: HashSet<Vec<Position>>,
}

impl Warehouse {
    fn get_box_checksum(&self) -> isize {
        let mut sum = 0;
        for box_positions in &self.boxes {
            let pos = box_positions.first().unwrap();
            sum += pos.x + pos.y * 100;
        }
        sum
    }

    fn contains_wall_at_pos(&self, pos: Position) -> bool {
        self.walls.contains(&pos)
    }

    fn contains_box_at_pos(&self, pos: Position) -> bool {
        self.contains_left_box_at_pos(pos)
            || self.contains_right_box_at_pos(pos)
            || self.boxes.contains(&vec![pos])
    }

    fn contains_left_box_at_pos(&self, pos: Position) -> bool {
        self.boxes
            .contains(&vec![pos.pos_in_dir(Instruction::Left), pos])
    }

    fn contains_right_box_at_pos(&self, pos: Position) -> bool {
        self.boxes
            .contains(&vec![pos, pos.pos_in_dir(Instruction::Right)])
    }

    fn is_empty_space(&self, pos: Position) -> bool {
        !self.contains_box_at_pos(pos) && !self.contains_wall_at_pos(pos) && self.robot != pos
    }

    fn get_next_possible_positions(
        &self,
        pos: Position,
        instruction: Instruction,
    ) -> Vec<Position> {
        if pos == self.robot {
            return vec![pos.pos_in_dir(instruction)];
        }

        let mut positions = vec![];

        let is_left_box = self.contains_left_box_at_pos(pos);
        let is_right_box = self.contains_right_box_at_pos(pos);
        let pos = pos.pos_in_dir(instruction);

        match instruction {
            Instruction::Left if is_left_box => positions.push(pos.pos_in_dir(Instruction::Left)),
            Instruction::Right if is_right_box => {
                positions.push(pos.pos_in_dir(Instruction::Right))
            }
            Instruction::Down if is_left_box || is_right_box => {
                positions.push(pos);
                positions.push(pos.pos_in_dir(if is_left_box {
                    Instruction::Left
                } else {
                    Instruction::Right
                }));
            }
            Instruction::Up if is_left_box || is_right_box => {
                positions.push(pos);
                positions.push(pos.pos_in_dir(if is_left_box {
                    Instruction::Left
                } else {
                    Instruction::Right
                }));
            }
            _ => positions.push(pos),
        }

        positions
    }

    fn move_box(&mut self, pos: Position, instruction: Instruction) {
        let is_box = self.boxes.contains(&vec![pos]);
        let is_left_box = self.contains_left_box_at_pos(pos);
        let is_right_box = self.contains_right_box_at_pos(pos);
        let new_pos = pos.pos_in_dir(instruction);
        match (is_box, is_left_box, is_right_box) {
            (_, true, false) => {
                self.boxes
                    .remove(&vec![pos.pos_in_dir(Instruction::Left), pos]);
                self.boxes
                    .insert(vec![new_pos.pos_in_dir(Instruction::Left), new_pos]);
            }
            (_, false, true) => {
                self.boxes
                    .remove(&vec![pos, pos.pos_in_dir(Instruction::Right)]);
                self.boxes
                    .insert(vec![new_pos, new_pos.pos_in_dir(Instruction::Right)]);
            }
            (true, _, _) => {
                self.boxes.remove(&vec![pos]);
                self.boxes.insert(vec![new_pos]);
            }
            _ => {}
        }
    }
}

pub struct Day15;

impl DaySolver for Day15 {
    fn part1(&self, input: &str) -> Option<String> {
        let (mut warehouse, instructions) = parse(input, false);
        for i in instructions {
            let robot = warehouse.robot;
            execute_move(&mut warehouse, robot, i);
        }
        Some(warehouse.get_box_checksum().to_string())
    }

    fn part2(&self, input: &str) -> Option<String> {
        let (mut warehouse, instructions) = parse(input, true);
        for i in instructions {
            let robot = warehouse.robot;
            if !discover(&warehouse, robot, i) {
                continue;
            }
            execute_move(&mut warehouse, robot, i);
        }
        Some(warehouse.get_box_checksum().to_string())
    }
}

fn execute_move(warehouse: &mut Warehouse, pos: Position, instruction: Instruction) -> bool {
    if warehouse.contains_wall_at_pos(pos) {
        return false;
    }

    if warehouse.is_empty_space(pos) {
        return true;
    }

    let next_pos = warehouse.get_next_possible_positions(pos, instruction);

    for n in next_pos {
        if !execute_move(warehouse, n, instruction) {
            return false;
        }
    }

    if pos == warehouse.robot {
        warehouse.robot = pos.pos_in_dir(instruction);
        return true;
    }

    warehouse.move_box(pos, instruction);
    true
}

/*
 * Discovers all possible movements and returns whether the robot can move in the direction given by the `instruction` (true) or not (false)
 */
fn discover(warehouse: &Warehouse, pos: Position, instruction: Instruction) -> bool {
    if warehouse.contains_wall_at_pos(pos) {
        return false;
    }

    if warehouse.is_empty_space(pos) {
        return true;
    }

    let next_pos = warehouse.get_next_possible_positions(pos, instruction);

    for n in next_pos {
        if !discover(warehouse, n, instruction) {
            return false;
        }
    }
    true
}

fn parse(input: &str, part2: bool) -> (Warehouse, Vec<Instruction>) {
    let input = input.replace("\r", "");
    let mut input = input.split("\n\n");
    let warehouse = input.next().unwrap();
    let instructions = input.next().unwrap().replace("\n", "");
    let mut robot = Position::new(-1, -1);
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();
    for (y, l) in warehouse.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if !part2 {
                parse_part1(
                    x as isize, y as isize, c, &mut walls, &mut boxes, &mut robot,
                );
            } else {
                parse_part2(
                    x as isize, y as isize, c, &mut walls, &mut boxes, &mut robot,
                );
            }
        }
    }
    let warehouse = Warehouse {
        robot,
        walls,
        boxes,
    };
    let instructions = instructions
        .chars()
        .map(|c| Instruction::from_str(&c.to_string()).unwrap())
        .collect();
    (warehouse, instructions)
}

fn parse_part1(
    x: isize,
    y: isize,
    c: char,
    walls: &mut HashSet<Position>,
    boxes: &mut HashSet<Vec<Position>>,
    robot: &mut Position,
) {
    match c {
        '#' => {
            walls.insert(Position::new(x, y));
        }
        'O' => {
            boxes.insert(vec![Position::new(x, y)]);
        }
        '@' => *robot = Position::new(x, y),
        _ => {}
    }
}

fn parse_part2(
    x: isize,
    y: isize,
    c: char,
    walls: &mut HashSet<Position>,
    boxes: &mut HashSet<Vec<Position>>,
    robot: &mut Position,
) {
    match c {
        '#' => {
            walls.insert(Position::new(2 * x, y));
            walls.insert(Position::new(2 * x + 1, y));
        }
        'O' => {
            boxes.insert(vec![Position::new(2 * x, y), Position::new(2 * x + 1, y)]);
        }
        '@' => *robot = Position::new(2 * x, y),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            ##########
            #..O..O.O#
            #......O.#
            #.OO..O.O#
            #..O@..O.#
            #O#..O...#
            #O..O..O.#
            #.OO.O.OO#
            #....O...#
            ##########

            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        "};
        let solver = Day15 {};
        assert_eq!(solver.part1(input).unwrap(), "10092");
        assert_eq!(solver.part2(input).unwrap(), "9021");
    }
}
