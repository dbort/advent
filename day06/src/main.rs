#[derive(Debug)]
struct Fish {
    timer: u8,
}

impl Fish {
    fn new(timer: u8) -> Self {
        Self { timer }
    }

    fn tick(&mut self) -> bool {
        if self.timer == 0 {
            self.timer = 6;
            true
        } else {
            self.timer -= 1;
            false
        }
    }
}

fn format_population(fishes: &Vec<Fish>) -> String {
    fishes
        .iter()
        .map(|f| format!("{}", f.timer))
        .collect::<Vec<String>>()
        .join(",")
}

/// Returns the number of fish after `days` days.
fn simulate(start: &Vec<u8>, days: usize) -> usize {
    let mut fish: Vec<Fish> = start.iter().map(|n| Fish::new(*n)).collect();
    println!("Initial state: {}", format_population(&fish));

    for _day in 1..=days {
        let mut new_fish: Vec<Fish> = vec![];
        for f in &mut fish {
            if f.tick() {
                new_fish.push(Fish::new(8));
            }
        }
        fish.append(&mut new_fish);
        // println!("After {} days: [{}] {}", day, fish.len(), format_population(&fish));
    }
    println!("Final: [{}]", fish.len());
    return fish.len();
}

fn first(input: &String) {
    let start: Vec<u8> = input.trim().split(',').map(|s| s.parse::<u8>().unwrap()).collect();
    simulate(&start, 80);
}

fn second(input: &String) {
    let start: Vec<u8> = input.trim().split(',').map(|s| s.parse::<u8>().unwrap()).collect();
    let mut total = 0;
    for (i, f) in start.iter().enumerate() {
        println!("{}/{}...", i, start.len());
        total += simulate(&vec![*f], 256);
    }
    println!("After 256 days: {}", total);
}

fn main() {
    let input = common::input::load_file("input.txt");
    first(&input);
    second(&input);
}
