advent_of_code::solution!(3);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Battery(u8);

impl From<char> for Battery {
    fn from(value: char) -> Self {
        Self(value.to_digit(10).unwrap() as u8)
    }
}

struct Bank(Vec<Battery>);

impl From<&str> for Bank {
    fn from(value: &str) -> Self {
        Self(value.chars().map(Battery::from).collect())
    }
}

impl Bank {
    fn highest_joltage<const N: usize>(&self) -> usize {
        // first find highest left to right
        let mut jolt_vals = [0; N];
        let mut cursor = 0;
        let bank_len = self.0.len();
        for i in 0..N {
            let minimum_keep = N - i - 1;
            let mut cur_max = 0;
            for (index, &b) in self
                .0
                .iter()
                .enumerate()
                .take(bank_len - minimum_keep)
                .skip(cursor)
            {
                if b.0 > cur_max {
                    cur_max = b.0;
                    cursor = index + 1;
                }
                if cur_max == 9 {
                    break;
                }
            }
            jolt_vals[i] = cur_max;
        }
        let mut f = 1;
        let mut total_jolts = 0;
        for v in jolt_vals.into_iter().rev() {
            total_jolts += v as usize * f;
            f *= 10;
        }
        total_jolts
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(Bank::from)
            .map(|bank| bank.highest_joltage::<2>())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(Bank::from)
            .map(|bank| bank.highest_joltage::<12>())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
