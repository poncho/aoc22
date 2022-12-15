use std::fs;

type Day8 = Vec<Vec<u32>>;

fn main() {
    let input = read_input("inputs/day8.txt");
    let p1_result = puzzle1(&input);
    let p2_result = puzzle2(&input);

    println!("Puzzle #1: {}", p1_result);
    println!("Puzzle #2: {}", p2_result);
}

fn read_input(path: &str) -> Day8 {
    let input = fs::read_to_string(path).unwrap();

    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn puzzle1(input: &Day8) -> usize {
    let mut visible_trees = 0;
    let column_size = input.len();
    let line_size = input.first().unwrap().len();

    for (line_idx, line) in input.iter().enumerate() {
        let mut highest_in_line = 0;

        for (column_idx, &tree) in line.iter().enumerate() {
            let mut outside_flag = false;

            if line_idx == 0
                || line_idx == line_size - 1
                || column_idx == 0
                || column_idx == column_size - 1
            {
                outside_flag = true;
            }

            if tree > highest_in_line {
                highest_in_line = tree;
                visible_trees += 1;
            } else if outside_flag
                || visible_from_top(input, line_idx, column_idx, tree)
                || visible_from_bottom(input, line_idx, column_idx, tree)
                || visible_from_right(input, line_idx, column_idx, tree)
            {
                visible_trees += 1;
            }
        }
    }

    visible_trees
}

fn visible_from_top(input: &Day8, line_idx: usize, column_idx: usize, current_tree: u32) -> bool {
    for line in input[..line_idx].iter() {
        if let Some(&tree) = line.get(column_idx) {
            if tree >= current_tree {
                return false;
            }
        }
    }

    true
}

fn visible_from_bottom(
    input: &Day8,
    line_idx: usize,
    column_idx: usize,
    current_tree: u32,
) -> bool {
    for line in input[line_idx + 1..].iter().rev() {
        if let Some(&tree) = line.get(column_idx) {
            if tree >= current_tree {
                return false;
            }
        }
    }

    true
}

fn visible_from_right(input: &Day8, line_idx: usize, column_idx: usize, current_tree: u32) -> bool {
    if let Some(line) = input.get(line_idx) {
        for &tree in line[column_idx + 1..].iter().rev() {
            if tree >= current_tree {
                return false;
            }
        }
    }

    true
}

fn puzzle2(input: &Day8) -> usize {
    let column_size = input.len();
    let line_size = input.first().unwrap().len();

    input
        .iter()
        .enumerate()
        .map(|(line_idx, line)| {
            line.iter()
                .enumerate()
                .map(|(column_idx, &tree)| {
                    if line_idx == 0
                        || line_idx == line_size - 1
                        || column_idx == 0
                        || column_idx == column_size - 1
                    {
                        0
                    } else {
                        let left = left_max_sight(&line[..column_idx], tree);
                        let right = right_max_sight(&line[column_idx + 1..], tree);
                        let top = top_max_sight(&input[..line_idx], column_idx, tree);
                        let bottom = bottom_max_sight(&input[line_idx + 1..], column_idx, tree);

                        top * bottom * left * right
                    }
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn left_max_sight(line: &[u32], tree: u32) -> usize {
    match line.iter().rev().enumerate().find(|(_index, &t)| t >= tree) {
        Some((distance, _)) => distance + 1,
        None => line.len(),
    }
}

fn right_max_sight(line: &[u32], tree: u32) -> usize {
    match line.iter().enumerate().find(|(_index, &t)| t >= tree) {
        Some((distance, _)) => distance + 1,
        None => line.len(),
    }
}

fn top_max_sight(slice: &[Vec<u32>], column_idx: usize, tree: u32) -> usize {
    for (line_idx, line) in slice.iter().rev().enumerate() {
        if let Some(t) = line.get(column_idx) {
            if *t >= tree {
                return line_idx + 1;
            }
        }
    }

    slice.len()
}

fn bottom_max_sight(slice: &[Vec<u32>], column_idx: usize, tree: u32) -> usize {
    for (line_idx, line) in slice.iter().enumerate() {
        if let Some(t) = line.get(column_idx) {
            if *t >= tree {
                return line_idx + 1;
            }
        }
    }

    slice.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Day8 {
        read_input("inputs/day8_test.txt")
    }

    #[test]
    fn puzzle1_test() {
        let test_input = test_input();

        assert_eq!(puzzle1(&test_input), 21)
    }

    #[test]
    fn puzzle2_test() {
        let test_input = test_input();

        assert_eq!(puzzle2(&test_input), 8)
    }
}
