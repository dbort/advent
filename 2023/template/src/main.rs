fn part1(input: &String) -> i64 {
    let mut sum: i64 = 0;
    for line in input.lines() {
        sum += 1;
    }
    sum
}

fn part2(input: &String) -> i64 {
    -1
}

fn main() {
    let input = std::fs::read_to_string("sample-input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
