use std::collections::HashMap;

use crate::utils::DaySolver;

struct Robot {
    pos: (isize, isize),
    v: (isize, isize),
}

pub struct Day14 {
    pub width: isize,
    pub height: isize,
}

impl DaySolver for Day14 {
    fn part1(&self, input: &str) -> Option<String> {
        let robots = parse(input);

        let mut upper_left = 0;
        let mut lower_right = 0;
        let mut lower_left = 0;
        let mut upper_right = 0;

        for r in &robots {
            let pos = self.get_pos(r, 100);
            if pos.0 < self.width / 2 && pos.1 < self.height / 2 {
                upper_left += 1;
            }
            if pos.0 > self.width / 2 && pos.1 > self.height / 2 {
                lower_right += 1;
            }
            if pos.0 < self.width / 2 && pos.1 > self.height / 2 {
                lower_left += 1;
            }
            if pos.0 > self.width / 2 && pos.1 < self.height / 2 {
                upper_right += 1;
            }
        }
        let safety_factor = upper_left * lower_right * lower_left * upper_right;

        Some(safety_factor.to_string())
    }

    fn part2(&self, input: &str) -> Option<String> {
        let robots = parse(input);
        let mut vertical_positions = HashMap::new();
        let mut horizontal_positions = HashMap::new();
        let mut seconds = -1;
        let mut offset_x = 0;
        let mut offset_y = 0;
        let limit = std::cmp::max(self.width, self.height);
        while seconds < limit {
            seconds += 1;
            horizontal_positions.clear();
            vertical_positions.clear();
            for r in &robots {
                let pos = self.get_pos(r, seconds);
                horizontal_positions
                    .entry(pos.1)
                    .and_modify(|e| *e += 1)
                    .or_insert(1_isize);
                vertical_positions
                    .entry(pos.0)
                    .and_modify(|e| *e += 1)
                    .or_insert(1_isize);
            }
            for y in 0..self.height {
                let Some(c) = horizontal_positions.get(&y) else {
                    continue;
                };
                if *c < self.width / 4 {
                    continue;
                }
                offset_y = seconds;
            }
            for x in 0..self.width {
                let Some(c) = vertical_positions.get(&x) else {
                    continue;
                };
                if *c < self.height / 4 {
                    continue;
                }
                offset_x = seconds;
            }
        }

        for i in 0..=self.width {
            for j in 0..=self.height {
                let t1 = offset_y + i * self.height;
                let t2 = offset_x + j * self.width;
                if t1 == t2 {
                    return Some(t1.to_string());
                }
            }
        }
        None
    }
}

impl Day14 {
    fn get_pos(&self, r: &Robot, seconds: isize) -> (isize, isize) {
        (
            (r.pos.0 + seconds * r.v.0).rem_euclid(self.width),
            (r.pos.1 + seconds * r.v.1).rem_euclid(self.height),
        )
    }
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|l| {
            let l = l.replace("p=", "").replace("v=", "");
            let mut l = l.split(" ");
            let mut pos = l.next().unwrap().split(",");
            let pos = (
                pos.next().unwrap().parse().unwrap(),
                pos.next().unwrap().parse().unwrap(),
            );
            let mut v = l.next().unwrap().split(",");
            let v = (
                v.next().unwrap().parse().unwrap(),
                v.next().unwrap().parse().unwrap(),
            );
            Robot { pos, v }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3
        "};
        let solver = Day14 {
            width: 11,
            height: 7,
        };
        assert_eq!(solver.part1(input).unwrap(), "12");
    }
}
