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
            if id.is_invalid_p1_alt() {
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
    #[allow(dead_code)]
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

    /// See [`Self::is_invalid_p2_alt`] documentation.
    ///
    /// Note: when the number can only be divided in two parts
    /// then this is faster than formatting an ascii string and comparing ascii bytes
    fn is_invalid_p1_alt(&self) -> bool {
        let n_pow10 = get_next_pow_of_10(self.0 + 1);
        let h_pow10 = n_pow10 / 2;
        let divider = 10_usize.pow(h_pow10 as u32);
        let lower_half = self.0 % divider;
        let higher_half = self.0 / divider;
        return lower_half == higher_half;
    }

    /// This implementation uses arithmetic to compare
    /// to manipulate the integers number
    ///
    /// Basically it computes the number of digits D of the number N
    /// and for every possible "pattern size" P (divider of the number of digits) it:
    /// 1. extract a pattern: pat=N % 10^P
    /// 2. reduces the current number and extract a to_match_pat=(N / 10^P) % 10^P
    /// 3. compares pat and to_match_pat,
    ///    if they match: continue to step2 until the current number reaches 0
    ///
    /// it divides and modulus the base number to check for matching
    ///
    ///
    /// Note: This happens to be slower than converting the number into
    /// ascii strings and comparing the ascii bytes.
    #[allow(dead_code)]
    fn is_invalid_p2_alt(&self) -> bool {
        let n_pow10 = get_next_pow_of_10(self.0 + 1);
        'size_loop: for i_pow10 in 1..=n_pow10 / 2 {
            if n_pow10 % i_pow10 != 0 {
                // i isn't a divider of n_pow10
                continue;
            }
            let pattern_amount = n_pow10 / i_pow10;
            let divider = 10_usize.pow(i_pow10 as u32);
            let mut current_number = self.0;
            let lowest_pat = current_number % divider;
            for _ in 1..pattern_amount {
                // "shift (base10)"
                current_number = current_number / divider;
                let pat = current_number % divider;
                if pat != lowest_pat {
                    continue 'size_loop;
                }
            }
            return true;
        }
        return false;
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

fn get_next_pow_of_10(n: usize) -> usize {
    // n^round(log_n(x))
    (n as f64).log10().ceil() as usize
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

    #[test]
    fn test_get_pow_of_10() {
        for (n, exp_pow) in [(9, 1), (11, 2), (10, 1), (2, 1), (101, 3), (1, 0)] {
            let pow = get_next_pow_of_10(n);
            assert_eq!(pow, exp_pow);
        }
    }
}
