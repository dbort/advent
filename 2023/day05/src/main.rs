use std::cmp::min;

#[derive(Debug, Clone, Copy)]
struct RangeMapping {
    src: usize,
    dst: usize,
    len: usize,
}

impl RangeMapping {
    fn parse(line: &str) -> Self {
        let parts: Vec<usize> = line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        assert!(parts.len() == 3, "Bad line '{}'", line);
        RangeMapping {
            src: parts[1],
            dst: parts[0],
            len: parts[2],
        }
    }
}

#[derive(Debug, Clone)]
struct ElementMapping {
    from: String,
    to: String,
    ranges: Vec<RangeMapping>,
}

impl ElementMapping {
    fn resolve(&self, input: usize) -> usize {
        for range in &self.ranges {
            if input >= range.src && input < range.src + range.len {
                return range.dst + input - range.src;
            }
        }
        return input;
    }
}

fn part1(input: &String) -> i64 {
    // Add a final line to flush the final record, shadowing the arg.
    let input = {
        let mut s = input.to_owned();
        s.push_str("\n");
        s
    };

    let mut mappings: Vec<ElementMapping> = vec![];

    let mut seeds: Vec<usize> = vec![];
    let mut reading = false;
    let mut from: &str = "";
    let mut to: &str = "";
    let mut ranges: Vec<RangeMapping> = vec![];
    for line in input.lines() {
        if line.starts_with("seeds:") {
            assert!(seeds.is_empty());
            seeds = line
                .strip_prefix("seeds:")
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            println!("seeds: {:?}", seeds);
        } else if line.is_empty() {
            if reading {
                assert!(!ranges.is_empty());
                assert!(!from.is_empty());
                assert!(!to.is_empty());
                mappings.push(ElementMapping {
                    from: from.to_owned(),
                    to: to.to_owned(),
                    ranges: ranges.clone(),
                });
            }
            ranges = vec![];
            from = "";
            to = "";
            reading = false;
        } else if line.ends_with(" map:") {
            (from, to) = line
                .strip_suffix(" map:")
                .and_then(|s| s.split_once("-to-"))
                .unwrap();
            reading = true;
        } else {
            ranges.push(RangeMapping::parse(line));
        }
    }
    // for m in mappings {
    //     println!("{:?}", m);
    // }

    let mut lowest_location = usize::MAX;
    for seed in seeds {
        let mut elem: &str = "seed";
        let mut val: usize = seed;
        while elem != "location" {
            let mapping: &ElementMapping = mappings.iter().find(|&m| m.from == elem).unwrap();
            elem = &mapping.to;
            val = mapping.resolve(val);
        }
        lowest_location = min(lowest_location, val);
    }
    lowest_location as i64
}

fn part2(input: &String) -> i64 {
    let _ = input;
    -1
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
