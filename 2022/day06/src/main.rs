use std::collections::HashSet;
use std::collections::VecDeque;

fn process(chars: &str, winlen: usize) -> usize {
    let mut window: VecDeque<char> = VecDeque::new();
    let mut cnum: usize = 0;
    for c in chars.chars() {
        cnum += 1;
        window.push_back(c);
        if window.len() > winlen {
            window.pop_front();
        }
        if window.len() == winlen {
            let winset: HashSet<&char> = HashSet::from_iter(window.iter());
            // println!("{cnum}: {:?}", winset);
            if winset.len() == winlen {
                return cnum;
            }
        }
    }
    return 0;
}

fn first(input: &String) {
    for line in input.lines() {
        println!("Index {}", process(line, 4));
    }
}

fn second(input: &String) {
    for line in input.lines() {
        println!("Index {}", process(line, 14));
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    first(&input);
    second(&input);
}
