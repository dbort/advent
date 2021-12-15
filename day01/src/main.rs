fn first(input: &String) {
    let mut prev: i32 = i32::MAX;
    let mut increases: i32 = 0;
    for line in input.lines() {
        let val: i32 = line.parse().unwrap();
        if val > prev {
            increases += 1;
        }
        prev = val;
    }
    println!("Single increases: {}", increases);
}

fn second(input: &String) {
    let mut increases: i32 = 0;
    let mut lines = 0;

    let mut window = [-1, -1, -1];
    for line in input.lines() {
        let prev: i32 = window.iter().sum();
        window[0] = window[1];
        window[1] = window[2];
        window[2] = line.parse().unwrap();
        let new: i32 = window.iter().sum();

        println!("pushed {} -- new {} vs prev {}", window[2], new, prev);
        lines += 1;
        if lines <= 3 {
            println!("skip");
            continue;
        }
        if new > prev {
            increases += 1;
        }
    }
    println!("Window increases: {}", increases);
}

fn main() {
    let input = common::input::load_file("input.txt");
    first(&input);
    second(&input);
}
