advent_of_code::solution!(2);

struct Range(usize, usize);

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let (min, max) = value.split_once("-").unwrap();
        let max = max.strip_suffix("\n").unwrap_or(max);
        Self(
            usize::from_str_radix(min, 10).unwrap(),
            usize::from_str_radix(max, 10).unwrap(),
        )
    }
}

impl Range {
    fn sum_invalid_ids_p1(&self) -> usize {
        let mut invalid = 0;
        for id in self.0..=self.1 {
            let id = Id(id);
            if id.is_invalid_p1() {
                invalid += id.0;
            }
        }
        invalid
    }

    fn sum_invalid_ids_p2(&self) -> usize {
        let mut invalid = 0;
        for id in self.0..=self.1 {
            let id = Id(id);
            if id.is_invalid_p2() {
                invalid += id.0;
            }
        }
        invalid
    }
}

#[derive(Debug)]
struct Id(usize);

impl Id {
    fn is_invalid_p1(&self) -> bool {
        let str_id = self.0.to_string();
        assert!(str_id.is_ascii());
        let str_bytes = str_id.as_bytes();
        let len = str_id.len();
        if len % 2 != 0 {
            // odd number of digits means no "twice repetition"
            return false;
        }
        let pattern_size = len / 2;
        return &str_bytes[0..pattern_size] == &str_bytes[pattern_size..];
    }

    fn is_invalid_p2(&self) -> bool {
        let str_id = self.0.to_string();
        assert!(str_id.is_ascii());
        let str_bytes = str_id.as_bytes();
        let len = str_id.len();
        'size_loop: for i /* potential repetition size */ in 1..=(len/2) {
            if len % i != 0 {
                // i isn't a divider of len
                continue;
            }
            // we allow ourselves to use as_bytes because we only use ascii
            let pattern = &str_bytes[..i];
            let pattern_amount = len / i;
            // that's always the case because of how i is constructed
            assert!(pattern_amount > 1);
            for n in 1..pattern_amount {
                if pattern != &str_bytes[n*i..(n+1) *i] {
                    continue 'size_loop;
                }
            }
            // pattern matched
            return true;
        }
        return false;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .split(",")
            .map(Range::from)
            .map(|r| r.sum_invalid_ids_p1())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .split(",")
            .map(Range::from)
            .map(|r| r.sum_invalid_ids_p2())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
