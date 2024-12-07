use crate::utils::DaySolver;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const TARGET: &str = "XMAS";

#[derive(EnumIter)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn is_feasible_part1(&self, width: isize, height: isize, x: isize, y: isize) -> bool {
        match self {
            Self::N if y < 3 => return false,
            Self::NE if y < 3 || x > (width - 4) => return false,
            Self::E if x > (width - 4) => return false,
            Self::SE if y > (height - 4) || x > (width - 4) => return false,
            Self::S if y > (height - 4) => return false,
            Self::SW if y > (height - 4) || x < 3 => return false,
            Self::W if x < 3 => return false,
            Self::NW if y < 3 || x < 3 => return false,
            _ => {}
        }
        true
    }
    fn test_occurence(&self, input: &[char], width: isize, x: isize, y: isize, z: bool) -> bool {
        let idx = (y * width + x) as usize;
        let target: Vec<char> = TARGET.chars().collect();
        let start = if z { 0 } else { 1 };

        for i in start..TARGET.len() - (1 - start) {
            match self {
                Self::N if input[idx - i * width as usize] != target[i + (1 - start)] => {
                    return false
                }
                Self::E if input[idx + i] != target[i + (1 - start)] => return false,
                Self::W if input[idx - i] != target[i + (1 - start)] => return false,
                Self::S if input[idx + i * width as usize] != target[i + (1 - start)] => {
                    return false
                }
                Self::NE if input[idx - i * width as usize + i] != target[i + (1 - start)] => {
                    return false
                }
                Self::NW if input[idx - i * width as usize - i] != target[i + (1 - start)] => {
                    return false
                }
                Self::SE if input[idx + i * width as usize + i] != target[i + (1 - start)] => {
                    return false
                }
                Self::SW if input[idx + i * width as usize - i] != target[i + (1 - start)] => {
                    return false
                }
                _ => {}
            }
        }
        true
    }
}

pub struct Day4;

impl DaySolver for Day4 {
    fn part1(&self, input: &str) -> Option<String> {
        let lines = input.lines();
        let height = lines.count();
        let mut lines = input.lines();
        let width = lines.next().unwrap().chars().count();
        let input: Vec<char> = input.replace("\n", "").replace("\r", "").chars().collect();

        let mut sum = 0;
        for y in 0..height {
            for x in 0..width {
                let c = input[y * width + x];
                if c == 'X' {
                    for i in Direction::iter() {
                        if !i.is_feasible_part1(
                            width as isize,
                            height as isize,
                            x as isize,
                            y as isize,
                        ) {
                            continue;
                        }
                        sum +=
                            i.test_occurence(&input, width as isize, x as isize, y as isize, false)
                                as usize;
                    }
                }
            }
        }

        Some(sum.to_string())
    }

    fn part2(&self, input: &str) -> Option<String> {
        let lines = input.lines();
        let height = lines.count();
        let mut lines = input.lines();
        let width = lines.next().unwrap().chars().count();
        let input: Vec<char> = input.replace("\n", "").replace("\r", "").chars().collect();

        let mut sum = 0;
        for y in 0..height {
            for x in 0..width {
                if y < 1 || x < 1 || y + 1 == height || x + 1 == width {
                    continue;
                }
                if input[y * width + x] != 'A' {
                    continue;
                }
                let mut counter = 0;
                counter += Direction::NW.test_occurence(
                    &input,
                    width as isize,
                    (x + 1) as isize,
                    (y + 1) as isize,
                    true,
                ) as usize;
                counter += Direction::SW.test_occurence(
                    &input,
                    width as isize,
                    (x + 1) as isize,
                    (y - 1) as isize,
                    true,
                ) as usize;
                counter += Direction::NE.test_occurence(
                    &input,
                    width as isize,
                    (x - 1) as isize,
                    (y + 1) as isize,
                    true,
                ) as usize;
                counter += Direction::SE.test_occurence(
                    &input,
                    width as isize,
                    (x - 1) as isize,
                    (y - 1) as isize,
                    true,
                ) as usize;

                if counter == 2 {
                    sum += 1;
                }
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
        let input = indoc! {"
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        "};
        let solver = Day4 {};
        assert_eq!(solver.part1(input).unwrap(), "18");

        let input = indoc! {"
            .M.S......
            ..A..MSMS.
            .M.S.MAA..
            ..A.ASMSM.
            .M.S.M....
            ..........
            S.S.S.S.S.
            .A.A.A.A..
            M.M.M.M.M.
            ..........
        "};
        assert_eq!(solver.part2(input).unwrap(), "9");
    }

    #[test]
    fn custom() {
        let input = indoc! {"
            X
            M
            A
            S
        "};
        let solver = Day4 {};
        assert_eq!(solver.part1(input).unwrap(), "1");
        //assert_eq!(solver.part2(input).unwrap(), "TODO");
    }
}
