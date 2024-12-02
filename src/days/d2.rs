use crate::utils::DaySolver;

const REPORT_DELIMITER: &str = " ";
const REPORT_MAX_DISTANCE: isize = 3;

pub struct Day2;

impl DaySolver for Day2 {
    fn part1(&self, input: &str) -> Option<String> {
        Some(
            input
                .lines()
                .fold(0, |acc, report| acc + is_report_safe(report, None) as usize)
                .to_string(),
        )
    }

    fn part2(&self, input: &str) -> Option<String> {
        Some(
            input
                .lines()
                .fold(0, |acc, report| {
                    for i in 0..report.split(REPORT_DELIMITER).count() {
                        if is_report_safe(report, Some(i)) {
                            return acc + 1;
                        }
                    }
                    acc
                })
                .to_string(),
        )
    }
}

fn is_report_safe(report: &str, skip: Option<usize>) -> bool {
    let mut prev_opt = None;
    let mut ascending_opt = None;
    for (i, n) in report.split(REPORT_DELIMITER).enumerate() {
        if let Some(skip) = skip {
            if i == skip {
                continue;
            }
        }

        let n: isize = n.parse().unwrap();
        let Some(prev) = prev_opt else {
            // first element, no previous element
            prev_opt = Some(n);
            continue;
        };
        prev_opt = Some(n);

        if (n - prev).abs() > REPORT_MAX_DISTANCE || n == prev {
            return false;
        }

        let Some(ascending) = ascending_opt else {
            ascending_opt = Some(prev < n);
            continue;
        };

        if ascending && prev >= n {
            return false;
        }
        if !ascending && prev <= n {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn sample() {
        let input = indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "};
        let solver = Day2 {};
        assert_eq!(solver.part1(input).unwrap(), "2");
        assert_eq!(solver.part2(input).unwrap(), "4");
    }
}
