use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct IdRange(usize, usize);

impl From<&str> for IdRange {
    fn from(value: &str) -> Self {
        let (l, r) = value.split_once("-").unwrap();
        Self(
            usize::from_str_radix(l, 10).unwrap(),
            usize::from_str_radix(r, 10).unwrap(),
        )
    }
}

enum MergeResult {
    Merged(IdRange),
    NotMerged,
}

impl IdRange {
    fn contains(&self, n: usize) -> bool {
        self.0 <= n && self.1 >= n
    }

    fn mergeable(&self, other: &Self) -> bool {
        self.contains(other.0)
            || self.contains(other.1)
            || other.contains(self.0)
            || other.contains(self.1)
    }

    fn merge(&self, other: &Self) -> MergeResult {
        if !self.mergeable(other) {
            return MergeResult::NotMerged;
        }

        MergeResult::Merged(Self(self.0.min(other.0), self.1.max(other.1)))
    }
}

struct Kitchen {
    ranges: Vec<IdRange>,
    ingredients: Vec<usize>,
}

impl From<&str> for Kitchen {
    fn from(value: &str) -> Self {
        let (ranges, ingredients) = value.split_once("\n\n").unwrap();
        Self {
            ranges: ranges.lines().map(IdRange::from).collect(),
            ingredients: ingredients
                .lines()
                .map(|l| usize::from_str_radix(l, 10).unwrap())
                .collect(),
        }
    }
}

impl Kitchen {
    fn count_fresh(&self) -> usize {
        let mut count = 0;
        for &ingredient in &self.ingredients {
            for range in &self.ranges {
                if range.contains(ingredient) {
                    count += 1;
                    break;
                }
            }
        }
        count
    }

    fn total_fresh(&self) -> usize {
        let mut merged_ranges: Vec<IdRange> = Vec::new();
        for &range in &self.ranges {
            merged_ranges.push(range);
        }

        // Most probably overkilled / contains dead code
        loop {
            let mut some_were_merged = false;
            let mut new_merged_ranges = Vec::new();
            let mut unmerged_set = HashSet::new();
            let mut merged_set = HashSet::new();
            for (r1, r2) in merged_ranges.iter().tuple_combinations() {
                if merged_set.contains(r1) || merged_set.contains(r2) {
                    continue;
                }
                match r1.merge(r2) {
                    MergeResult::NotMerged => {
                        unmerged_set.insert(r1);
                        unmerged_set.insert(r2);
                    }
                    MergeResult::Merged(mr) => {
                        merged_set.insert(r1);
                        merged_set.insert(r2);
                        new_merged_ranges.push(mr);
                        some_were_merged = true;
                    }
                }
            }
            if !some_were_merged {
                break;
            }
            for mr in merged_set {
                unmerged_set.remove(mr);
            }
            for remaining in unmerged_set {
                assert!(!new_merged_ranges.contains(remaining));
                new_merged_ranges.push(*remaining);
            }
            merged_ranges = new_merged_ranges;
        }

        let mut total_fresh = 0;
        for r in merged_ranges {
            total_fresh += r.1 - r.0 + 1;
        }
        total_fresh
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let kitchen = Kitchen::from(input);
    Some(kitchen.count_fresh())
}

pub fn part_two(input: &str) -> Option<usize> {
    let kitchen = Kitchen::from(input);
    Some(kitchen.total_fresh())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
