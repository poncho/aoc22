use std::{collections::HashMap, fs};

type Day7 = Vec<String>;

#[derive(Debug)]
struct AocFile {
    id: usize,
    name: String,
    dir: usize,
    size: usize,
}

impl AocFile {}

struct System {
    current_dir: usize,
    files: Vec<AocFile>,
    file_id_count: usize,
}

impl System {
    fn get_next_id(&mut self) -> usize {
        self.file_id_count += 1;

        self.file_id_count
    }

    fn mkdir(&mut self, name: &str) {
        let file = AocFile {
            id: self.get_next_id(),
            name: name.to_owned(),
            size: 0,
            dir: self.current_dir,
        };

        self.files.push(file)
    }

    fn add_file(&mut self, name: &str, size: usize) {
        let file = AocFile {
            id: self.get_next_id(),
            name: name.to_owned(),
            size,
            dir: self.current_dir,
        };

        self.files.push(file)
    }
    fn cd(&mut self, path: &str) {
        match path {
            ".." => match self.files.iter_mut().find(|f| f.id == self.current_dir) {
                Some(dir) => self.current_dir = dir.dir,
                _ => panic!("Parent dir of {} not found", self.current_dir),
            },
            _ => match self
                .files
                .iter()
                .find(|f| f.name == path && f.dir == self.current_dir)
            {
                Some(dir) => self.current_dir = dir.id,
                _ => panic!("Dir {} not found", path),
            },
        }
    }
}

fn main() {
    let input = read_input("inputs/day7.txt");
    let p1_result = puzzle1(&input);

    println!("Puzzle #1: {}", p1_result);
}

fn read_input(path: &str) -> Day7 {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|s| s.to_owned())
        .collect()
}

fn puzzle1(input: &Day7) -> usize {
    let mut system = System {
        current_dir: 0,
        files: vec![],
        file_id_count: 0,
    };

    system.mkdir("/");
    input.iter().for_each(|l| run_line(&mut system, l));

    let mut group: HashMap<usize, (usize, usize)> = HashMap::new();

    system.files.iter().for_each(|f| {
        group
            .entry(f.dir)
            .and_modify(|value| value.0 += f.size)
            .or_insert({
                match system.files.iter().find(|p| p.id == f.dir) {
                    Some(parent) => (f.size, parent.dir),
                    _ => (f.size, 0),
                }
            });
    });

    let mut sorted_groups = group
        .clone()
        .into_iter()
        .map(|(dir_id, _)| dir_id)
        .collect::<Vec<usize>>();

    sorted_groups.sort_by_key(|v| *v);

    for dir_id in sorted_groups.iter().rev() {
        let (dir_size, parent_id) = group[dir_id];

        group
            .entry(parent_id)
            .and_modify(|value| value.0 += dir_size);
    }

    group
        .iter()
        .filter(|&(_k, v)| v.0 <= 100_000)
        .map(|(_k, v)| v.0)
        .sum()
}

fn run_line(system: &mut System, line: &str) {
    let mut args = line.split_whitespace();

    match args.next().unwrap() {
        "$" => run_command(system, args),
        "dir" => {
            let dir_name = args.next().unwrap();
            system.mkdir(dir_name);
        }
        size => {
            let file_name = args.next().unwrap();
            let file_size: usize = size.parse().unwrap();

            system.add_file(file_name, file_size);
        }
    }
}

fn run_command(system: &mut System, mut args: std::str::SplitWhitespace) {
    match args.next().unwrap() {
        "cd" => {
            let dir_to_find = args.next().unwrap();

            system.cd(dir_to_find)
        }
        "ls" => (),
        invalid_command => {
            panic!("Invalid command: {}", invalid_command)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle1_test() {
        let test_input = read_input("inputs/day7_test.txt");

        assert_eq!(puzzle1(&test_input), 95437);
    }
}
