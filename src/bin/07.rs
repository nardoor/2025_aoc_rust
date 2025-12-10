use std::collections::{BTreeMap, BTreeSet, HashMap};

advent_of_code::solution!(7);

#[derive(Debug)]
struct TachyonManyfold {
    start: usize,
    splitters: Vec<Vec<usize>>,
}

impl From<&str> for TachyonManyfold {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let start = lines.next().unwrap().find("S").unwrap();
        let splitters = lines
            .map(|l| {
                l.chars()
                    .enumerate()
                    .filter_map(|(i, c)| (c == '^').then_some(i))
                    .collect()
            })
            .filter(|v: &Vec<usize>| !v.is_empty())
            .collect();
        Self { start, splitters }
    }
}

impl TachyonManyfold {
    fn count_tachyon_split(&self) -> usize {
        let mut current_tachyons = BTreeSet::new();
        current_tachyons.insert(self.start);
        let mut split_count = 0;
        for splitter_line in &self.splitters {
            let mut new_current_tachyons = BTreeSet::new();
            for tachyon in &current_tachyons {
                if splitter_line.contains(&tachyon) {
                    new_current_tachyons.insert(tachyon - 1);
                    new_current_tachyons.insert(tachyon + 1);
                    split_count += 1;
                } else {
                    new_current_tachyons.insert(*tachyon);
                }
            }
            current_tachyons = new_current_tachyons;
        }
        split_count
    }
    fn count_tachyon_path(
        &self,
        tachyon_x: usize,
        tachyon_y: usize,
        cache: &mut BTreeMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(cache_path_count) = cache.get(&(tachyon_x, tachyon_y)) {
            return *cache_path_count;
        }
        let mut cur_tachyon_y = tachyon_y;
        loop {
            let Some(splitters) = self.splitters.get(cur_tachyon_y) else {
                // reached bottom
                return 1;
            };
            if splitters.contains(&tachyon_x) {
                let count_a = self.count_tachyon_path(tachyon_x - 1, cur_tachyon_y + 1, cache);
                let count_b = self.count_tachyon_path(tachyon_x + 1, cur_tachyon_y + 1, cache);
                let path_count = count_a + count_b;
                cache.insert((tachyon_x, tachyon_y), path_count);
                return path_count;
            } else {
                cur_tachyon_y += 1;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let tachyon_manyfold = TachyonManyfold::from(input);
    Some(tachyon_manyfold.count_tachyon_split())
}

pub fn part_two(input: &str) -> Option<usize> {
    let tachyon_manyfold = TachyonManyfold::from(input);
    let mut cache = BTreeMap::new();
    let tachyon_x = tachyon_manyfold.start;
    let tachyon_y = 0;
    Some(tachyon_manyfold.count_tachyon_path(tachyon_x, tachyon_y, &mut cache))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
