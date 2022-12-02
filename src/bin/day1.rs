use std::fs;

type Day1 = Vec<String>;

const TOP_SIZE: usize = 3;

fn main() {
    let input = read_input("inputs/day1.txt");
    let p1_result = puzzle1(&input);
    let p2_result = puzzle2(&input);

    println!("Puzzle #1: {}", p1_result);
    println!("Puzzle #2: {}", p2_result);
}

fn read_input(path: &str) -> Day1 {
    let input = fs::read_to_string(path).unwrap();

    input.lines().map(|l| l.to_owned()).collect()
}

fn puzzle1(input: &Day1) -> u32 {
    let mut max: u32 = 0;
    let mut current: u32 = 0;

    for line in input.iter() {
        match line.parse::<u32>() {
            Ok(value) => current += value,
            _ => {
                max = if current > max { current } else { max };
                current = 0;
            }
        }
    }

    if current > max {
        current
    } else {
        max
    }
}

fn puzzle2(input: &Day1) -> u32 {
    let mut top: [u32; TOP_SIZE] = [0, 0, 0];
    let mut current: u32 = 0;

    for line in input.iter() {
        match line.parse::<u32>() {
            Ok(value) => current += value,
            _ => {
                if current > top[TOP_SIZE - 1] {
                    add_to_top(&mut top, current)
                }
                current = 0;
            }
        }
    }

    if current > top[TOP_SIZE - 1] {
        add_to_top(&mut top, current)
    }

    top.iter().sum()
}

fn add_to_top(top: &mut [u32; TOP_SIZE], new_value: u32) {
    top[TOP_SIZE - 1] = new_value;

    top.to_owned()
        .iter()
        .enumerate()
        .rev()
        .skip(1)
        .for_each(|(index, &value)| {
            if new_value > value {
                top[index + 1] = value;
                top[index] = new_value;
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Vec<String> {
        read_input("inputs/day1_test.txt")
    }

    #[test]
    fn puzzle1_test() {
        let test_input = test_input();

        assert_eq!(puzzle1(&test_input), 24000)
    }

    #[test]
    fn puzzle2_test() {
        let test_input = test_input();
        assert_eq!(puzzle2(&test_input), 45000)
    }
}
