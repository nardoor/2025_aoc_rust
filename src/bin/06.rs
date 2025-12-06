advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy)]
enum Operation {
    Sum,
    Multiply,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Sum,
            "*" => Self::Multiply,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Calculus {
    nums: Vec<usize>,
    op: Operation,
}

impl Calculus {
    fn new(nums: Vec<usize>, op: Operation) -> Self {
        Self { op, nums }
    }

    fn solve(&self) -> usize {
        match self.op {
            Operation::Multiply => self.nums.iter().fold(1, |acc, el| acc * el),
            Operation::Sum => self.nums.iter().sum(),
        }
    }
}

fn parse_p1(input: &str) -> Vec<Calculus> {
    let mut lines = input.lines();
    let mut num_lines = Vec::new();
    let mut ops = Vec::new();
    while let Some(l) = lines.next() {
        if l.contains("+") {
            ops = l
                .split(" ")
                .filter(|&n| !n.is_empty())
                .map(Operation::from)
                .collect();
            continue;
        }
        num_lines.push(
            l.split(" ")
                .filter(|&n| !n.is_empty())
                .map(|n| usize::from_str_radix(n, 10).unwrap())
                .collect::<Vec<usize>>(),
        );
    }

    for i in 0..num_lines.len() {
        assert_eq!(ops.len(), num_lines[i].len());
    }

    let mut calculus_vec = Vec::new();

    for i in 0..ops.len() {
        calculus_vec.push(Calculus::new(
            num_lines.iter().map(|nums| nums[i]).collect(),
            ops[i],
        ));
    }

    calculus_vec
}

fn parse_p2(input: &str) -> Vec<Calculus> {
    let lines = input.lines().collect::<Vec<&str>>();
    let len = lines.iter().map(|l| l.len()).max().unwrap() + 1;
    let mut op = None;
    let mut nums = Vec::new();
    let mut calculus_vec = Vec::new();
    for i in 0..len {
        let mut l = lines
            .iter()
            .map(|&l| l.chars().nth(i).unwrap_or(' '))
            .collect::<String>();

        if l.trim().is_empty() {
            calculus_vec.push(Calculus::new(nums.clone(), op.unwrap()));
            nums.clear();
            op = None;
            continue;
        }
        if l.ends_with("+") {
            op = Some(Operation::Sum);
            l.remove(l.find("+").unwrap());
        } else if l.ends_with("*") {
            op = Some(Operation::Multiply);
            l.remove(l.find("*").unwrap());
        }
        nums.push(usize::from_str_radix(l.trim(), 10).unwrap());
    }
    calculus_vec
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(parse_p1(input).into_iter().map(|c| c.solve()).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(parse_p2(input).into_iter().map(|c| c.solve()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
