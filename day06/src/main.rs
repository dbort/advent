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

fn first(input: &String) {
    for line in input.lines() {
        let start: Vec<u8> = line.split(',').map(|s| s.parse::<u8>().unwrap()).collect();

        let mut fish: Vec<Fish> = start.iter().map(|n| Fish::new(*n)).collect();
        println!("Initial state: {}", format_population(&fish));

        for _day in 1..=80 {
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
    }
}

fn second(_input: &String) {
}

fn main() {
    let input = common::input::load_file("input.txt");
    first(&input);
    second(&input);
}
