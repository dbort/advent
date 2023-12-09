fn line_val(line: &str) -> u32 {
    let mut first: u32 = 10;
    let mut last: u32 = 10;
    for c in line.chars() {
        if c.is_digit(10) {
            last = c.to_digit(10).unwrap();
            if first > 9 {
                first = last;
            }
        }
    }
    assert!(first <= 9, "No digits in '{}'", line);
    return first * 10 + last;
}

fn first(input: &String) {
    let mut sum: u64 = 0;
    for line in input.lines() {
        sum += line_val(line) as u64;
    }
    println!("First total: {}", sum);
}

struct Number {
    word: String,
    digit: i8,
}

fn second(input: &String) {
    let numbers: Vec<_> = vec![
        Number {
            word: "one".to_string(),
            digit: 1,
        },
        Number {
            word: "two".to_string(),
            digit: 2,
        },
        Number {
            word: "three".to_string(),
            digit: 3,
        },
        Number {
            word: "four".to_string(),
            digit: 4,
        },
        Number {
            word: "five".to_string(),
            digit: 5,
        },
        Number {
            word: "six".to_string(),
            digit: 6,
        },
        Number {
            word: "seven".to_string(),
            digit: 7,
        },
        Number {
            word: "eight".to_string(),
            digit: 8,
        },
        Number {
            word: "nine".to_string(),
            digit: 9,
        },
    ];
    let mut sum: u64 = 0;
    for linei in input.lines() {
        let mut first: u32 = 10;
        let mut line: &str = linei;
        'outer: while line.len() > 0 {
            let c = line.chars().nth(0).unwrap();
            if c.is_digit(10) {
                first = c.to_digit(10).unwrap();
                break;
            } else {
                for n in &numbers {
                    if line.starts_with(&n.word) {
                        first = n.digit as u32;
                        break 'outer;
                    }
                }
            }
            line = &line[1..];
        }

        let mut last: u32 = 10;
        'outer: for i in (0..linei.len()).rev() {
            line = &linei[i..];
            println!("<{}>", line);
            let c = line.chars().nth(0).unwrap();
            if c.is_digit(10) {
                last = c.to_digit(10).unwrap();
                break 'outer;
            } else {
                for n in &numbers {
                    if line.starts_with(&n.word) {
                        last = n.digit as u32;
                        break 'outer;
                    }
                }
            }
        }

        assert!(first < 10 && last < 10, "Didn't find numbers for {}", line);
        let val = first as u64 * 10 + last as u64;
        println!("{} -> {}", linei, val);
        sum += first as u64 * 10 + last as u64;
    }
    println!("Second total: {}", sum);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // first(&input);
    second(&input);
}
