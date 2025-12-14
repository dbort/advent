use std::collections::HashSet;
use std::ops::RangeInclusive;

fn parse_range_line(line: &str) -> RangeInclusive<u64> {
    let mut parts = line.split('-');

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
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, HashSet<u64>) {
    let mut sections = input.splitn(2, "\n\n");
    let ranges_section = sections.next().unwrap();
    let values_section = sections.next().unwrap();

    let ranges: Vec<RangeInclusive<u64>> = ranges_section
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_range_line(line)) // Each item is now Result<RangeInclusive<u64>, String>
        .collect::<Vec<_>>();

    let values: HashSet<u64> = values_section
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().parse::<u64>().unwrap())
        .collect::<HashSet<_>>();

    (ranges, values)
}

fn part1(input: &String) -> i64 {
    let mut count = 0;
    let (ranges, values) = parse_input(input);
    'outer: for value in &values {
        for range in &ranges {
            if range.contains(value) {
                count += 1;
                continue 'outer;
            }
        }
    }
    count
}

fn part2(input: &String) -> i64 {
    let (nonmut_ranges, _) = parse_input(input);
    let mut ranges = nonmut_ranges.clone();
    ranges.sort_unstable_by_key(|r| *r.start());
    let mut p = 0;
    for i in 1..ranges.len() {
        if ranges[i].start() >= ranges[p].start() && ranges[i].start() <= ranges[p].end() {
            ranges[p] = RangeInclusive::new(
                *ranges[p].start(),
                std::cmp::max(*ranges[p].end(), *ranges[i].end()),
            );
            ranges[i] = RangeInclusive::new(0, 0);
        } else {
            p = i;
        }
    }
    let mut count = 0;
    for range in ranges {
        println!(
            "{}..={} ({})",
            range.start(),
            range.end(),
            range.end() - range.start() + 1
        );
        if (*range.start(), *range.end()) != (0, 0) {
            count += range.end() - range.start() + 1;
        }
    }
    count as i64
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
