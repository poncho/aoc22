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
    let directories = list_directories(&input);

    let p1_result = puzzle1(&directories);
    let p2_result = puzzle2(&directories);

    println!("Puzzle #1: {}", p1_result);
    println!("Puzzle #2: {}", p2_result);
}

fn read_input(path: &str) -> Day7 {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|s| s.to_owned())
        .collect()
}

fn puzzle1(directories: &HashMap<usize, (usize, usize)>) -> usize {
    directories
        .iter()
        .filter(|&(_k, v)| v.0 <= 100_000)
        .map(|(_k, v)| v.0)
        .sum()
}

fn puzzle2(directories: &HashMap<usize, (usize, usize)>) -> usize {
    let total_disk_space = 70000000;
    let required_space_for_update = 30000000;
    let (used_space, _parent) = directories[&1];

    println!("USED SPACE: {}", used_space);

    let needed_free_space = required_space_for_update - (total_disk_space - used_space);

    directories
        .values()
        .filter(|(size, _)| *size >= needed_free_space)
        .map(|(size, _)| *size)
        .min()
        .unwrap()
}

fn list_directories(input: &Day7) -> HashMap<usize, (usize, usize)> {
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

    fn test_input() -> HashMap<usize, (usize, usize)> {
        let input = read_input("inputs/day7_test.txt");
        list_directories(&input)
    }

    #[test]
    fn puzzle1_test() {
        assert_eq!(puzzle1(&test_input()), 95437);
    }

    #[test]
    fn puzzle2_test() {
        assert_eq!(puzzle2(&test_input()), 24933642);
    }
}
