use std::{num::ParseIntError, str::FromStr};

advent_of_code::solution!(1);

#[derive(Debug)]
enum Rotation {
    L(usize),
    R(usize),
}

impl Rotation {
    fn rot(&self) -> usize {
        match self {
            Self::L(rot) | Self::R(rot) => *rot,
        }
    }

    fn delta(&self) -> isize {
        match *self {
            Self::L(rot) => -(rot as isize),
            Self::R(rot) => rot as isize,
        }
    }

    /// Returns (pos0_count, remaining delta)
    fn effective_delta(&self) -> (usize, isize) {
        let pos0_count = self.delta().abs() as usize / 100;
        let new_rot = (self.rot().rem_euclid(100)) as isize;
        let effective_delta = match self {
            Self::L(_) => -new_rot,
            Self::R(_) => new_rot,
        };
        (pos0_count, effective_delta)
    }
}

impl FromStr for Rotation {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, rot) = s.split_at(1);
        let rot = usize::from_str_radix(rot, 10)?;
        match dir {
            "L" => Ok(Self::L(rot)),
            "R" => Ok(Self::R(rot)),
            _ => panic!("Unexpected letter!"),
        }
    }
}

#[derive(Debug)]
struct Dial(isize);

impl Dial {
    fn new() -> Self {
        Self(50)
    }

    /// Returns current pos
    fn rotate_p1(&mut self, rotation: Rotation) -> isize {
        let new_pos = (self.0 + rotation.delta()) % 100;
        self.0 = new_pos;
        new_pos
    }

    /// Returns 0 count for this rotation
    fn rotate_p2(&mut self, rotation: Rotation) -> usize {
        let (mut pos0_count, effective_delta) = rotation.effective_delta();
        let new_pos = self.0 + effective_delta;

        if self.0 != 0 && (new_pos <= 0 || new_pos > 99) {
            pos0_count += 1;
        }

        self.0 = new_pos.rem_euclid(100);
        pos0_count
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let rotations = input.lines().map(|l| Rotation::from_str(l).unwrap());
    let mut dial = Dial::new();

    let mut pos0_count = 0;

    for rot in rotations {
        if dial.rotate_p1(rot) == 0 {
            pos0_count += 1;
        }
    }

    Some(pos0_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let rotations = input.lines().map(|l| Rotation::from_str(l).unwrap());
    let mut dial = Dial::new();

    let mut pos0_count = 0;

    for rot in rotations {
        pos0_count += dial.rotate_p2(rot);
    }

    Some(pos0_count)
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
        assert_eq!(result, Some(6));
    }
}
