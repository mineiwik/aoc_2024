use std::collections::HashMap;

use crate::utils::DaySolver;

pub struct Day11;

impl DaySolver for Day11 {
    fn part1(&self, input: &str) -> Option<String> {
        let input = input.replace("\n", "").replace("\r", "");
        let stones: Vec<usize> = input.split(" ").map(|e| e.parse().unwrap()).collect();

        let mut sum = 0;
        let mut dp = HashMap::new();
        for s in stones {
            sum += explore(s, &mut dp, 25);
        }

        Some(sum.to_string())
    }

    fn part2(&self, input: &str) -> Option<String> {
        let input = input.replace("\n", "").replace("\r", "");
        let stones: Vec<usize> = input.split(" ").map(|e| e.parse().unwrap()).collect();

        let mut sum = 0;
        let mut dp = HashMap::new();
        for s in stones {
            sum += explore(s, &mut dp, 75);
        }

        Some(sum.to_string())
    }
}

fn explore(stone: usize, dp: &mut HashMap<(usize, usize), usize>, steps_rem: usize) -> usize {
    if steps_rem == 0 {
        return 1;
    }

    if let Some(sum) = dp.get(&(stone, steps_rem)) {
        return *sum;
    }
    let num_digits = stone.checked_ilog10().unwrap_or(0) as usize + 1;
    let sum = match stone {
        0 => explore(1, dp, steps_rem - 1),
        _ if num_digits % 2 == 0 => {
            let pow = 10_i32.pow(num_digits as u32 / 2) as usize;
            let l: usize = stone / pow;
            let r: usize = stone % pow;
            explore(l, dp, steps_rem - 1) + explore(r, dp, steps_rem - 1)
        }
        _ => explore(stone * 2024, dp, steps_rem - 1),
    };
    dp.insert((stone, steps_rem), sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"125 17"};
        let solver = Day11 {};
        assert_eq!(solver.part1(input).unwrap(), "55312");
    }
}
