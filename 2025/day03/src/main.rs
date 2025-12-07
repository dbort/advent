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

// ndigits = 8
// 1891111119 (len 10)
// 0..3 -> 0 .. len - (digits_left - 1)
// --> pos 1 val 8, left = 7
// pos+1 .. (len - pos+1) - (left - 1)
fn part2(input: &String) -> i64 {
    let mut sum: i64 = 0;
    let ndigits = 12;
    for line in input.lines() {
        let digits = parse_digits(line);
        let mut value = 0;
        let mut pos = 0;
        println!("in: {}", line);
        for digits_left in (1..=ndigits).rev() {
            let avail = digits.len() - pos;
            let look = avail - digits_left + 1;
            let mp = max_position(&digits[pos..pos + look]) + pos;
            pos = mp + 1;
            value = value * 10 + digits[mp] as i64;
            print!("[{}]:{}, ", mp, digits[mp]);
        }
        println!("\nval: {}", value);
        sum += value as i64;
    }
    sum
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
