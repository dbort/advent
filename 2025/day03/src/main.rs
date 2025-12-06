fn parse_digits(s: &str) -> Vec<u8> {
    s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
}

fn max_position(nums: &[u8]) -> usize {
    let mut max = 0;
    let mut max_index = 0;
    for i in 0..nums.len() {
        if nums[i] > max {
            max = nums[i];
            max_index = i;
        }
    }
    max_index
}

fn part1(input: &String) -> i64 {
    let mut sum: i64 = 0;
    for line in input.lines() {
        let digits = parse_digits(line);
        let mp1 = max_position(&digits[0..digits.len() - 1]);
        let mp2 = max_position(&digits[mp1 + 1..]) + mp1 + 1;
        let value = digits[mp1] * 10 + digits[mp2];
        println!("{} -> [{},{}] {}", line, mp1, mp2, value);
        sum += value as i64;
    }
    sum
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
