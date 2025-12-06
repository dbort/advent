use std::ops::RangeInclusive;

fn parse_ranges(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .trim()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|range_str| {
            let mut parts = range_str.split('-');

            let start = parts
                .next()
                .unwrap()
                .parse::<u64>()
                .expect("Failed to parse start number");

            let end = parts
                .next()
                .unwrap()
                .parse::<u64>()
                .expect("Failed to parse end number");

            start..=end
        })
        .collect()
}

fn part1(input: &String) -> i64 {
    let ranges = parse_ranges(input);
    let mut total = 0;
    for range in ranges {
        // println!("{:?}", range);
        for i in range {
            if ((i as f32).log10() as u32) % 2 == 1 {
                // println!("log cand.: {} ({})", i, (i as f32).log10() as u32);
                let s = i.to_string();
                let p1 = &s[0..s.len() / 2];
                let p2 = &s[s.len() / 2..s.len()];
                // println!("parts: {}:{}", p1, p2);
                if p1 == p2 {
                    println!("match: {}:{}", p1, p2);
                    total += i;
                }
            }
        }
    }
    total.try_into().unwrap()
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
