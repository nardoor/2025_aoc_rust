use std::{
    collections::HashSet,
    fmt::{Debug, Write},
};

advent_of_code::solution!(4);

#[derive(PartialEq, Eq, Clone, Copy)]
struct PaperRoll;

impl PaperRoll {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '@' => Some(Self),
            '.' => None,
            _ => panic!(),
        }
    }
}

impl Debug for PaperRoll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('@')
    }
}

struct Grid {
    grid: Vec<Vec<Option<PaperRoll>>>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        Self {
            grid: value
                .lines()
                .map(|l| l.chars().map(PaperRoll::from_char).collect())
                .collect(),
        }
    }
}

impl Grid {
    fn get_x_len(&self) -> usize {
        self.grid[0].len()
    }
    fn get_y_len(&self) -> usize {
        self.grid.len()
    }
    fn get_tile(&self, x: usize, y: usize) -> Option<PaperRoll> {
        self.grid[y][x]
    }

    fn remove_tile(&mut self, x: usize, y: usize) {
        assert!(self.get_tile(x, y).is_some());
        self.grid[y][x] = None;
    }

    fn get_around_pos(&self, x: usize, y: usize) -> [Option<(usize, usize)>; 8] {
        let mut around_pos = [None; 8];
        let mut i = 0;
        let max_x = self.get_x_len() - 1;
        let max_y = self.get_y_len() - 1;
        for dx in [0, -1, 1] {
            for dy in [0, -1, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let Some(nx) = x.checked_add_signed(dx) else {
                    continue;
                };
                let Some(ny) = y.checked_add_signed(dy) else {
                    continue;
                };
                if nx > max_x || ny > max_y {
                    continue;
                }
                around_pos[i] = Some((nx, ny));
                i += 1;
            }
        }
        around_pos
    }

    fn verify(&self, x: usize, y: usize) -> bool {
        let mut paper_count = 0;
        for (nx, ny) in self.get_around_pos(x, y).into_iter().filter_map(|p| p) {
            if let Some(PaperRoll) = self.get_tile(nx, ny) {
                paper_count += 1;
            }
            if paper_count >= 4 {
                return false;
            }
        }
        true
    }

    fn count_verified(&self) -> usize {
        let mut count = 0;
        for x in 0..self.get_x_len() {
            for y in 0..self.get_y_len() {
                if self.get_tile(x, y).is_some() && self.verify(x, y) {
                    count += 1;
                }
            }
        }
        count
    }

    fn remove_possible(&mut self) -> usize {
        let mut cache = HashSet::new();
        let mut removed = 0;

        // first loop, init cache
        for x in 0..self.get_x_len() {
            for y in 0..self.get_y_len() {
                if self.get_tile(x, y).is_some() && self.verify(x, y) {
                    self.get_around_pos(x, y)
                        .into_iter()
                        .filter_map(|p| p)
                        .filter(|p| self.get_tile(p.0, p.1).is_some())
                        .for_each(|p| {
                            cache.insert(p);
                        });
                    self.remove_tile(x, y);
                    removed += 1;
                }
            }
        }

        loop {
            let mut tile_removed = false;
            let mut new_cache = HashSet::new();
            for &(x, y) in &cache {
                // x,y is a paper_roll (by cache construction)
                if self.get_tile(x, y).is_none() {
                    continue;
                }
                if self.verify(x, y) {
                    self.get_around_pos(x, y)
                        .into_iter()
                        .filter_map(|p| p)
                        .filter(|p| self.get_tile(p.0, p.1).is_some())
                        .for_each(|p| {
                            new_cache.insert(p);
                        });
                    self.remove_tile(x, y);
                    tile_removed = true;
                    removed += 1;
                } else {
                    new_cache.insert((x, y));
                }
            }
            if !tile_removed {
                break;
            }
            cache = new_cache;
            // new_cache.into_iter().for_each(|p| {
            // cache.insert(p);
            // });
        }

        removed
    }
}

// impl Debug for Grid {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut out = String::new();
//         for y in 0..self.get_y_len() {

//             for x in 0..self.get_x_len() {

//             }
//         }
//     }
// }

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::from(input);
    Some(grid.count_verified())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = Grid::from(input);
    Some(grid.remove_possible())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
