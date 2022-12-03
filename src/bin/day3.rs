use std::fs;

type Day3 = Vec<Vec<char>>;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let input = read_input("inputs/day3.txt");
    let p1_result = puzzle1(&input);

    println!("Puzzle #1: {}", p1_result);
}

fn read_input(path: &str) -> Day3 {
    let input = fs::read_to_string(path).unwrap();

    input.lines().map(|l| l.chars().collect()).collect()
}

fn puzzle1(input: &Day3) -> usize {
    let priorities: Vec<(usize, char)> = ALPHABET.char_indices().collect();

    input
        .iter()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);

            first
                .iter()
                .find(|&c| second.iter().any(|c2| c2 == c))
                .unwrap()
                .to_owned()
        })
        .map(|c| {
            let (index, _) = priorities
                .iter()
                .find(|(_index, letter)| c == *letter)
                .unwrap();

            *index + 1
        })
        .sum()
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
}
