use std::fs;

type Day2 = Vec<(char, char)>;

#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_code(letter_code: char) -> Self {
        match letter_code {
            move_code if move_code == 'A' || move_code == 'X' => Move::Rock,
            move_code if move_code == 'B' || move_code == 'Y' => Move::Paper,
            _ => Move::Scissors,
        }
    }

    fn play(&self, move2: &Self) -> u32 {
        match (self, move2) {
            (move1, move2) if move1 == move2 => 3,
            (Move::Rock, Move::Paper) => 6,
            (Move::Paper, Move::Scissors) => 6,
            (Move::Scissors, Move::Rock) => 6,
            _ => 0,
        }
    }

    fn points(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

fn main() {
    let input: Day2 = read_input("inputs/day2.txt");
    let p1_result = puzzle1(&input);

    println!("Puzzle #1: {}", p1_result);
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
    for (op_move, response) in input.iter() {
        let your_response = Move::from_code(*response);
        let round_points = Move::from_code(*op_move).play(&your_response) + your_response.points();

        total_points += round_points;
    }

    total_points
}

fn to_move_type(move_code: &str) -> Move {
    match move_code {
        move_code if move_code == "A" || move_code == "X" => Move::Rock,
        move_code if move_code == "B" || move_code == "Y" => Move::Paper,
        _ => Move::Scissors,
    }
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
}
