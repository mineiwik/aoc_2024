use crate::utils::DaySolver;

struct Machine {
    button_a: (f64, f64),
    button_b: (f64, f64),
    prize: (f64, f64),
}
pub struct Day13;

impl DaySolver for Day13 {
    fn part1(&self, input: &str) -> Option<String> {
        let machines = parse(input);
        let mut sum = 0;

        for m in machines {
            let Some((a, b)) = get_solution(&m) else {
                continue;
            };
            sum += a as isize * 3 + b as isize;
        }

        Some(sum.to_string())
    }

    fn part2(&self, input: &str) -> Option<String> {
        let mut machines = parse(input);
        for m in &mut machines {
            m.prize.0 += 10000000000000.0;
            m.prize.1 += 10000000000000.0;
        }
        let mut sum = 0;

        for m in machines {
            let Some((a, b)) = get_solution(&m) else {
                continue;
            };
            sum += a as isize * 3 + b as isize;
        }

        Some(sum.to_string())
    }
}

/*
 * Simple Linear Algebra:
 * 2 unknowns, 2 equations -> we will always get a solution
 * however, the solution might be negative
 * a * a_0 + b * b_0 = p_0
 * a * a_1 + b * b_1 = p_1
 *
 * a = (p_0 - b * b_0) / a_0
 * b * b_1 - b * b_0 / a_0 * a_1 = p_1 - p_0 / a_0 * a_1
 * b = (p_1 - p_0 / a_0 * a_1) / (b_1 - b_0 / a_0 * a_1)
 */
fn get_solution(m: &Machine) -> Option<(usize, usize)> {
    let b = (m.prize.1 - m.prize.0 / m.button_a.0 * m.button_a.1)
        / (m.button_b.1 - m.button_b.0 / m.button_a.0 * m.button_a.1);
    let a = (m.prize.0 - b * m.button_b.0) / m.button_a.0;

    if a < 0.0 || b < 0.0 {
        return None;
    }

    let a = a.round();
    let b = b.round();

    if a * m.button_a.0 + b * m.button_b.0 != m.prize.0 {
        return None;
    }

    if a * m.button_a.1 + b * m.button_b.1 != m.prize.1 {
        return None;
    }

    Some((a as usize, b as usize))
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .replace("\r", "")
        .split("\n\n")
        .map(|machine| {
            let mut machine = machine.lines();
            let button_a = machine
                .next()
                .unwrap()
                .replace("Button A: ", "")
                .replace("X+", "")
                .replace("Y+", "");
            let mut button_a = button_a.split(", ");
            let button_a = (
                button_a.next().unwrap().parse().unwrap(),
                button_a.next().unwrap().parse().unwrap(),
            );
            let button_b = machine
                .next()
                .unwrap()
                .replace("Button B: ", "")
                .replace("X+", "")
                .replace("Y+", "");
            let mut button_b = button_b.split(", ");
            let button_b = (
                button_b.next().unwrap().parse().unwrap(),
                button_b.next().unwrap().parse().unwrap(),
            );
            let prize = machine
                .next()
                .unwrap()
                .replace("Prize: ", "")
                .replace("X=", "")
                .replace("Y=", "");
            let mut prize = prize.split(", ");
            let prize = (
                prize.next().unwrap().parse().unwrap(),
                prize.next().unwrap().parse().unwrap(),
            );
            Machine {
                button_a,
                button_b,
                prize,
            }
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
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400

            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176

            Button A: X+17, Y+86
            Button B: X+84, Y+37
            Prize: X=7870, Y=6450

            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279
        "};
        let solver = Day13 {};
        assert_eq!(solver.part1(input).unwrap(), "480");
    }
}
