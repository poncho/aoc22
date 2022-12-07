use std::fs;

type Day5 = (Vec<Vec<char>>, Vec<Movement>);

#[derive(Debug)]
struct Movement {
    from: usize,
    to: usize,
    amount: usize,
}

fn main() {
    let (state, movements): Day5 = read_input("inputs/day5.txt");
    let p1_result = puzzle1(state.clone(), &movements);
    let p2_result = puzzle2(state, &movements);

    println!("Puzzle #1: {}", p1_result);
    println!("Puzzle #2: {}", p2_result);
}

fn read_input(path: &str) -> Day5 {
    let input = fs::read_to_string(path).unwrap();

    let mut split = input.split("\n\n");
    let state_data = split.next().unwrap();
    let moves_data = split.next().unwrap();

    (parse_state(state_data), parse_moves(moves_data))
}

fn parse_state(state_data: &str) -> Vec<Vec<char>> {
    let mut state: Vec<Vec<char>> = vec![];
    let mut iter = state_data.split('\n').rev();

    iter.next()
        .unwrap()
        .split_whitespace()
        .for_each(|_| state.push(vec![]));

    iter.for_each(|l| {
        let total_stacks = state.len();
        let mut current_stack = 0;

        loop {
            if current_stack + 1 > total_stacks {
                break;
            }

            let current_index = current_stack * 4;
            let current = &l[current_index..current_index + 3];

            match current.chars().nth(1) {
                Some(value) if value.is_alphabetic() => {
                    state[current_stack].push(value);
                }
                _ => (),
            }

            current_stack += 1;
        }
    });

    state
}

fn parse_moves(moves: &str) -> Vec<Movement> {
    moves
        .lines()
        .map(|m| {
            let mut split = m
                .split_whitespace()
                .map(|w| w.parse::<usize>())
                .filter(|w| w.is_ok());

            let amount = split.next().unwrap().unwrap();
            let from = split.next().unwrap().unwrap();
            let to = split.next().unwrap().unwrap();

            Movement { from, to, amount }
        })
        .collect()
}

fn puzzle1(mut state: Vec<Vec<char>>, movements: &[Movement]) -> String {
    for m in movements.iter() {
        for _ in 0..m.amount {
            let crate_code = state[m.from - 1].pop().unwrap();

            state[m.to - 1].push(crate_code);
        }
    }

    state.iter().map(|stack| stack.last().unwrap()).collect()
}

fn puzzle2(mut state: Vec<Vec<char>>, movements: &[Movement]) -> String {
    for m in movements.iter() {
        let new_length = state[m.from - 1].len() - m.amount;
        let mut crates_to_move = state[m.from - 1].split_off(new_length);

        state[m.to - 1].append(&mut crates_to_move);
    }

    state.iter().map(|stack| stack.last().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> Day5 {
        read_input("inputs/day5_test.txt")
    }

    #[test]
    fn puzzle1_test() {
        let (state, movements) = test_input();

        assert_eq!(puzzle1(state, &movements), "CMZ")
    }

    #[test]
    fn puzzle2_test() {
        let (state, movements) = test_input();

        assert_eq!(puzzle2(state, &movements), "MCD")
    }
}
