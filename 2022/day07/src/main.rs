use std::collections::HashMap;

#[derive(Debug)]
enum Node {
    File(usize),
    Directory(HashMap<String, DirEnt>),
}

#[derive(Debug)]
struct DirEnt {
    name: String,
    node: Node,
}

impl DirEnt {
    fn new(spec: &str) -> Self {
        let parts: Vec<&str> = spec.split(' ').collect();
        assert!(parts.len() == 2);
        DirEnt {
            name: parts[1].to_string(),
            node: match parts[0] {
                "dir" => Node::Directory(HashMap::new()),
                _ => Node::File(parts[0].parse().unwrap()),
            },
        }
    }
}

fn build(input: &String) -> DirEnt {
    let mut root = DirEnt::new("dir /");
    let mut cwd: Vec<&mut DirEnt> = vec![&mut root];
    for line in input.lines() {
        if line.as_bytes()[0] == '$' as u8 {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() == 3 {
                assert!(parts[0] == "$");
                assert!(parts[1] == "cd");
                if parts[2] == ".." {
                    cwd.pop();
                } else if parts[2] != "/" {
                    panic!("TODO: look for {}", parts[2]);
                }
            }
        } else {
            let mut dirent = DirEnt::new(line);
            println!("'{}' --> {:?}", line, dirent);
            match &mut cwd.last().unwrap().node {
                Node::Directory(mut ents) => {
                    ents.insert(dirent.name, dirent);
                }
                _ => {
                    panic!("file in cwd vec")
                }
            }
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
