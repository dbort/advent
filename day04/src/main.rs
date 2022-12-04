// use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
struct Range {
    min: i64,
    max: i64,
}

impl Range {
    fn new(spec: &str) -> Self {
        let row: Vec<i64> = spec
            .split('-')
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        if row.len() != 2 {
            panic!("Malformed range spec {}", spec);
        }
        Range {
            min: row[0],
            max: row[1],
        }
    }

    fn width(&self) -> i64 {
        self.max - self.min + 1
    }
}

#[derive(Debug)]
struct Pair {
    first: Range,
    second: Range,
}

impl Pair {
    fn new(spec: &str) -> Self {
        let ranges: Vec<Range> = spec
            .split(',')
            .filter(|s| s.len() > 0)
            .map(|s| Range::new(s))
            .collect();
        if ranges.len() != 2 {
            panic!("Malformed pair spec {}", spec);
        }
        Pair {
            first: ranges[0],
            second: ranges[1],
        }
    }

    fn full_overlap(&self) -> bool {
        let (bigger, smaller) = {
            if self.first.width() > self.second.width() {
                (self.first, self.second)
            } else {
                (self.second, self.first)
            }
        };
        bigger.min <= smaller.min && bigger.max >= smaller.max
    }

    fn overlaps(&self) -> bool {
        let (lower, higher) = {
            if self.first.min < self.second.min {
                (self.first, self.second)
            } else {
                (self.second, self.first)
            }
        };
        lower.max >= higher.min
    }
}

fn first(input: &String) {
    let mut sum: i64 = 0;
    for line in input.lines() {
        let pair = Pair::new(line);
        // println!("{:?}", pair);
        if pair.full_overlap() {
            sum += 1;
        }
    }
    println!("Total: {}", sum);
}

fn second(input: &String) {
    let mut sum: i64 = 0;
    for line in input.lines() {
        let pair = Pair::new(line);
        if pair.overlaps() {
            sum += 1;
        }
    }
    println!("Total: {}", sum);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    first(&input);
    second(&input);
}
