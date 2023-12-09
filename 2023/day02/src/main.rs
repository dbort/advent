#[derive(Debug)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

impl Round {
    fn parse(line: &str) -> Self {
        let parts: Vec<&str> = line.split(", ").collect();
        let mut red: usize = 0;
        let mut green: usize = 0;
        let mut blue: usize = 0;
        for part in &parts {
            let subparts: Vec<&str> = part.split(' ').collect();
            assert!(
                subparts.len() == 2,
                "Couldn't parse '{}' in '{}'",
                part,
                line
            );
            let count: usize = subparts[0].parse::<usize>().unwrap();
            match subparts[1] {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => panic!("unknown color '{}' in '{}'", subparts[1], line),
            }
        }
        Round { red, green, blue }
    }
}

fn parse_game(line: &str) -> (usize, Vec<Round>) {
    let parts: Vec<&str> = line.split(": ").collect();
    assert!(parts.len() == 2, "Couldn't parse '{}'", line);
    let game: usize = {
        let subparts: Vec<&str> = parts[0].split(' ').collect();
        assert!(subparts.len() == 2, "Couldn't parse '{}'", line);
        assert!(subparts[0] == "Game");
        subparts[1].parse::<usize>().unwrap()
    };
    let rounds = parts[1].split("; ").map(Round::parse).collect();
    (game, rounds)
}

fn first(input: &String) {
    let mut sum: usize = 0;
    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;
    'outer: for line in input.lines() {
        let (game, rounds) = parse_game(line);
        for round in rounds {
            if round.red > MAX_RED {
                continue 'outer;
            }
            if round.green > MAX_GREEN {
                continue 'outer;
            }
            if round.blue > MAX_BLUE {
                continue 'outer;
            }
        }
        sum += game;
    }
    println!("Total: {}", sum);
}

fn second(input: &String) {}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    first(&input);
    second(&input);
}
