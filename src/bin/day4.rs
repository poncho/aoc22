use std::{fs, str::Split};

type Day4 = Vec<([u16; 4])>;

fn main() {
    let input = read_input("inputs/day4.txt");
    let p1_result = puzzle1(&input);
    let p2_result = puzzle2(&input);

    println!("Puzzle #1: {}", p1_result);
    println!("Puzzle #2: {}", p2_result);
}

fn read_input(path: &str) -> Day4 {
    let input = fs::read_to_string(path).unwrap();

    input
        .lines()
        .map(|l| {
            let mut split = l.split(&['-', ',']);

            [
                next_and_convert(&mut split),
                next_and_convert(&mut split),
                next_and_convert(&mut split),
                next_and_convert(&mut split),
            ]
        })
        .collect()
}

fn next_and_convert(split: &mut Split<&[char; 2]>) -> u16 {
    split.next().unwrap().parse::<u16>().unwrap()
}

fn puzzle1(input: &Day4) -> usize {
    input
        .iter()
        .filter(|[r1_start, r1_end, r2_start, r2_end]| {
            r1_start <= r2_start && r1_end >= r2_end || r2_start <= r1_start && r2_end >= r1_end
        })
        .count()
}

fn puzzle2(input: &Day4) -> usize {
    input
        .iter()
        .filter(|[r1_start, r1_end, r2_start, r2_end]| {
            (r1_start <= r2_start && r2_start <= r1_end)
                || (r2_start <= r1_start && r1_start <= r2_end)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Day4 {
        read_input("inputs/day4_test.txt")
    }

    #[test]
    fn puzzle1_test() {
        let test_input = test_input();

        assert_eq!(puzzle1(&test_input), 2)
    }

    #[test]
    fn puzzle2_test() {
        let test_input = test_input();

        assert_eq!(puzzle2(&test_input), 4)
    }
}
