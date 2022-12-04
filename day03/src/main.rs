use std::collections::HashSet;

fn priority(c: char) -> i64 {
    match c {
        'a'..='z' => c as i64 - 'a' as i64 + 1,
        'A'..='Z' => c as i64 - 'A' as i64 + 27,
        _ => panic!("Unhandled character {}", c),
    }
}

#[derive(Debug)]
struct Compartment {
    contents: HashSet<char>,
}

impl Compartment {
    fn new(chars: &str) -> Self {
        Compartment {
            contents: HashSet::from_iter(chars.chars()),
        }
    }
}

#[derive(Debug)]
struct Rucksack {
    c1: Compartment,
    c2: Compartment,
}

impl Rucksack {
    fn new(line: &str) -> Self {
        let len = line.len();
        if len % 2 != 0 {
            panic!("Odd line length {} for {}", len, line);
        }
        let div = len / 2;
        Rucksack {
            c1: Compartment::new(&line[..div]),
            c2: Compartment::new(&line[div..]),
        }
    }

    fn overlap(&self) -> HashSet<char> {
        self.c1
            .contents
            .intersection(&self.c2.contents)
            .copied()
            .collect()
    }
}

fn first(input: &String) {
    let mut sum: i64 = 0;
    for line in input.lines() {
        let rs = Rucksack::new(line);
        let overlap_set = rs.overlap();
        let overlap: Vec<char> = overlap_set.iter().copied().collect();
        if overlap.len() != 1 {
            println!("{:?} from {}", rs, line);
            panic!(
                "Bad overlap {:?} with size {} for line {}",
                overlap,
                overlap.len(),
                line
            );
        }
        sum += priority(overlap[0])
    }
    println!("Total: {}", sum);
}

fn second(input: &String) {}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    first(&input);
    second(&input);
}
