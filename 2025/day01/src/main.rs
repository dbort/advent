#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Step {
    direction: Direction,
    distance: u32,
}

fn parse_steps(input: &str) -> Vec<Step> {
    input
        .lines()
        // Filter out empty lines just in case
        .filter(|line| !line.is_empty())
        .map(|line| {
            let line = line.trim();

            // Split the string at index 1
            // "L50" becomes "L" and "50"
            let (dir_str, num_str) = line.split_at(1);

            let direction = match dir_str {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Unknown direction encountered: {}", dir_str),
            };

            let distance = num_str.parse::<u32>().expect("Failed to parse number");

            Step {
                direction,
                distance,
            }
        })
        .collect()
}

fn part1(input: &String) -> i64 {
    let steps = parse_steps(input);
    let mut dial: i64 = 50;
    let mut zero_count = 0;
    for step in &steps {
        let sign = match &step.direction {
            Direction::Left => -1,
            Direction::Right => 1,
        };
        dial = dial + sign * step.distance as i64;
        while dial < 0 {
            dial += 100;
        }
        dial = dial % 100;
        println!("{:?} -> {}", step, dial);
        if dial == 0 {
            zero_count += 1
        }
    }
    zero_count
}

fn part2(input: &String) -> i64 {
    let _ = input;
    -1
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename> [<part>]", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    let part = if args.len() >= 3 {
        args[2].parse::<u32>().expect("Failed to parse part number")
    } else {
        3
    };

    println!("Using input from {}", filename);
    let input = std::fs::read_to_string(filename).unwrap();

    if part == 1 || part > 2 {
        println!("Part 1: {}", part1(&input));
    }
    if part == 2 || part > 2 {
        println!("Part 2: {}", part2(&input));
    }
}
