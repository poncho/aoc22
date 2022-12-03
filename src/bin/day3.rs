use std::fs;

type Day3 = Vec<Vec<char>>;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let input = read_input("inputs/day3.txt");
    let p1_result = puzzle1(&input);
    let p2_result = puzzle2(&input);

    println!("Puzzle #1: {}", p1_result);
    println!("Puzzle #2: {}", p2_result);
}

fn read_input(path: &str) -> Day3 {
    let input = fs::read_to_string(path).unwrap();

    input.lines().map(|l| l.chars().collect()).collect()
}

fn puzzle1(input: &Day3) -> usize {
    input
        .iter()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);

            let repeated = get_repeated(first, second);

            get_priority(repeated)
        })
        .sum()
}

fn puzzle2(input: &Day3) -> usize {
    input
        .chunks(3)
        .map(|chunk| {
            let repeated: Vec<char> = chunk[0]
                .iter()
                .copied()
                .filter(|&c| chunk[1].iter().any(|&c2| c2 == c))
                .collect();

            let badge = get_repeated(&repeated, &chunk[2]);

            get_priority(badge)
        })
        .sum()
}

fn get_repeated(first: &[char], second: &[char]) -> char {
    first
        .iter()
        .find(|&&c| second.iter().any(|&c2| c2 == c))
        .unwrap()
        .to_owned()
}

fn get_priority(element: char) -> usize {
    let priorities: Vec<(usize, char)> = ALPHABET.char_indices().collect();

    let (index, _) = priorities
        .iter()
        .find(|(_index, letter)| element == *letter)
        .unwrap();

    *index + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Day3 {
        read_input("inputs/day3_test.txt")
    }

    #[test]
    fn puzzle1_test() {
        let test_input = test_input();

        assert_eq!(puzzle1(&test_input), 157)
    }

    #[test]
    fn puzzle2_test() {
        let test_input = test_input();

        assert_eq!(puzzle2(&test_input), 70)
    }
}
