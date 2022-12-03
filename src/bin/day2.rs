use std::fs;

type Day2 = Vec<(char, char)>;

#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_code(letter_code: char) -> Self {
        match letter_code {
            move_code if move_code == 'A' || move_code == 'X' => Self::Rock,
            move_code if move_code == 'B' || move_code == 'Y' => Self::Paper,
            _ => Self::Scissors,
        }
    }

    fn play(&self, move2: &Self) -> u32 {
        match (self, move2) {
            (move1, move2) if move1 == move2 => 3,
            (Self::Rock, Self::Paper) => 6,
            (Self::Paper, Self::Scissors) => 6,
            (Self::Scissors, Self::Rock) => 6,
            _ => 0,
        }
    }

    fn points(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn winning_response(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn losing_response(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

fn main() {
    let input: Day2 = read_input("inputs/day2.txt");
    let p1_result = puzzle1(&input);
    let p2_result = puzzle2(&input);

    println!("Puzzle #1: {}", p1_result);
    println!("Puzzle #2: {}", p2_result);
}

fn read_input(path: &str) -> Day2 {
    let input = fs::read_to_string(path).unwrap();

    input
        .lines()
        .map(|l| {
            let mut split = l.split_whitespace();
            let op_move = split.next().unwrap().chars().next().unwrap();
            let response = split.next().unwrap().chars().next().unwrap();

            (op_move, response)
        })
        .collect::<Day2>()
}

fn puzzle1(input: &Day2) -> u32 {
    let mut total_points: u32 = 0;
    for (op_move_code, response_code) in input.iter() {
        let response = Move::from_code(*response_code);
        let round_points = Move::from_code(*op_move_code).play(&response) + response.points();

        total_points += round_points;
    }

    total_points
}

fn puzzle2(input: &Day2) -> u32 {
    input
        .iter()
        .map(|(op_move_code, result_code)| {
            let op_move = Move::from_code(*op_move_code);

            let response = match result_code {
                'X' => op_move.losing_response(),
                'Y' => op_move,
                _ => op_move.winning_response(),
            };

            op_move.play(&response) + response.points()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Day2 {
        read_input("inputs/day2_test.txt")
    }

    #[test]
    fn puzzle1_test() {
        let test_input = test_input();

        assert_eq!(puzzle1(&test_input), 15)
    }

    #[test]
    fn puzzle2_test() {
        let test_input = test_input();

        assert_eq!(puzzle2(&test_input), 12)
    }
}
