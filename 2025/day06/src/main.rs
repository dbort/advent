fn parse(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let lines: Vec<&str> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    let num_lines = lines.len();
    let char_line = lines[num_lines - 1];
    let number_lines = &lines[0..num_lines - 1];

    let characters: Vec<char> = char_line
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();

    let num_cols = number_lines[0].split_whitespace().count();
    let mut columns: Vec<Vec<u64>> = vec![Vec::new(); num_cols];

    for line in number_lines {
        let numbers = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        for (col_index, num) in numbers.into_iter().enumerate() {
            columns[col_index].push(num);
        }
    }

    (columns, characters)
}

fn part1(input: &String) -> i64 {
    let mut total = 0;
    let (cols, chars) = parse(input);
    for i in 0..chars.len() {
        total += match chars[i] {
            '+' => cols[i].iter().sum::<u64>(),
            '*' => cols[i].iter().product::<u64>(),
            _ => panic!(),
        }
    }
    total as i64
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
