fn first(input: &String) {
    let mut sum: i64 = 0;
    for line in input.lines() {
        sum += 1;
    }
    println!("Total: {}", sum);
}

fn second(input: &String) {}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    first(&input);
    second(&input);
}
