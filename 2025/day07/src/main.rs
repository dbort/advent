use std::collections::HashSet;

fn get_indexed_characters(line: &str) -> Vec<(usize, char)> {
    line.chars()
        .enumerate() // (index, character) tuples
        .filter(|&(_, c)| c != '.')
        .collect()
}

fn part1(input: &String) -> i64 {
    let mut beams = HashSet::new();
    let mut splits: i64 = 0;
    for line in input.lines() {
        let char_indices = get_indexed_characters(line);
        if char_indices.len() == 1 && char_indices[0].1 == 'S' {
            beams.insert(char_indices[0].0);
            println!("Found initial beam at {}", char_indices[0].0);
            continue;
        }
        for (i, _c) in char_indices {
            if beams.contains(&i) {
                beams.remove(&i);
                beams.insert(i - 1);
                beams.insert(i + 1);
                splits += 1;
            }
        }
    }
    splits
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
