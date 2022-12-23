#[allow(dead_code, unused_variables)]

#[derive(Debug)]
struct Display {
    inputs: Vec<String>,
    output: Vec<String>,
}

impl Display {
    fn new(line: &str) -> Self {
        let parts: Vec<String> = line.split(" | ").map(|s| s.to_string()).collect();
        assert_eq!(parts.len(), 2);
        Display {
            inputs: parts[0].split(' ').map(|s| s.to_string()).collect(),
            output: parts[1].split(' ').map(|s| s.to_string()).collect(),
        }
    }
}

fn first(input: &String) {
    let mut count = 0;
    for line in input.lines() {
        let display = Display::new(line);
        for d in display.output {
            match d.len() {
                7 | 4 | 3 | 2 => count += 1,
                _ => (),
            }
        }
    }
    println!("Count: {}", count);
}

fn second(input: &String) {
    for _line in input.lines() {
    }
}

fn main() {
    let input = common::input::load_file("input.txt");
    first(&input);
    second(&input);
}
