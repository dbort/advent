fn first(input: &String) {
    let mut sum: u64 = 0;
    for line in input.lines() {
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
        let value: u32 = first * 10 + last;
        sum += value as u64;
    }
    println!("Total: {}", sum);
}

fn second(input: &String) {}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    first(&input);
    second(&input);
}
