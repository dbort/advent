fn first(input: &String) {
    let mut horiz = 0;
    let mut depth = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let direction = parts[0];
        let value: i32 = parts[1].parse().unwrap();

        match direction {
            "forward" => horiz += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => panic!("Unknown direction {}", direction),
        }
    }

    println!("horiz {} x depth {} = {}", horiz, depth, horiz * depth);
}

fn second(input: &String) {
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let direction = parts[0];
        let value: i32 = parts[1].parse().unwrap();

        match direction {
            "forward" => { horiz += value; depth += aim * value },
            "down" => aim += value,
            "up" => aim -= value,
            _ => panic!("Unknown direction {}", direction),
        }
        println!("{} => aim {}, horiz {}, depth {}", line, aim, horiz, depth);
    }

    println!("horiz {} x depth {} = {}", horiz, depth, horiz * depth);
}

fn main() {
    let input = common::input::load_file("input.txt");
    first(&input);
    second(&input);
}
