use std::collections::HashMap;

#[derive(Debug)]
enum DirEnt {
    File(usize),
    Directory(HashMap<String, DirEnt>),
}

impl DirEnt {
    fn add_entry(&mut self, spec: &str) {
        if let DirEnt::Directory(ref mut map) = self {
            let parts: Vec<&str> = spec.split(' ').collect();
            assert!(parts.len() == 2);
            let name = String::from(parts[1]);
            map.insert(
                name,
                match parts[0] {
                    "dir" => DirEnt::Directory(HashMap::new()),
                    _ => DirEnt::File(parts[0].parse().unwrap()),
                },
            );
        } else {
            panic!("DirEnt is not a directory");
        }
    }

    fn cd(&mut self, name: &str) -> &mut DirEnt {
        if let DirEnt::Directory(ref mut map) = self {
            return map.get_mut(name).unwrap();
        } else {
            panic!("DirEnt is not a directory");
        }
    }
}

// https://blog.rust-lang.org/2015/04/17/Enums-match-mutation-and-moves.html

fn build(input: &String) -> DirEnt {
    let mut root = DirEnt::Directory(HashMap::new());
    let mut path: Vec<&mut DirEnt> = vec![&mut root];
    for line in input.lines() {
        if line.as_bytes()[0] == '$' as u8 {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() == 3 {
                assert!(parts[0] == "$");
                assert!(parts[1] == "cd");
                if parts[2] == ".." {
                    path.pop();
                } else if parts[2] != "/" {
                    let lasti = path.len() - 1;
                    let cwd = &mut path[lasti];
                    path.push(cwd.cd(parts[2]));
                }
            }
        } else {
            let lasti = path.len() - 1;
            let cwd = &mut path[lasti];
            println!("Adding: {} to {:?}", line, cwd);
            cwd.add_entry(line);
        }
    }
    root
}

fn first(input: &String) {
    let root = build(input);
}

fn second(input: &String) {}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    first(&input);
    second(&input);
}
