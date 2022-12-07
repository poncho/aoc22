use std::fs;

type Day6 = String;

fn main() {
    let input = read_input("inputs/day6.txt");
    let p1_result = puzzle1(&input);

    println!("Puzzle #1: {}", p1_result);
}

fn read_input(path: &str) -> Day6 {
    fs::read_to_string(path).unwrap()
}

fn puzzle1(input: &Day6) -> usize {
    let signal_length = input.len();

    for i in 0..signal_length {
        let current = &input[i..i + 4];

        let all_diff = current.chars().enumerate().all(|(i, c)| {
            let repeated = current
                .chars()
                .enumerate()
                .take_while(|(i2, _)| *i2 != i)
                .any(|(_, c2)| c2 == c);

            !repeated
        });

        if all_diff {
            return i + 4;
        }
    }

    signal_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle1_test() {
        let test_input_1 = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_owned();
        let test_input_2 = "nppdvjthqldpwncqszvftbrmjlhg".to_owned();
        let test_input_3 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_owned();
        let test_input_4 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_owned();

        assert_eq!(puzzle1(&test_input_1), 5);
        assert_eq!(puzzle1(&test_input_2), 6);
        assert_eq!(puzzle1(&test_input_3), 10);
        assert_eq!(puzzle1(&test_input_4), 11);
    }
}
