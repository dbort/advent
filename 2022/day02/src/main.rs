#[derive(Copy, Clone, Debug)]
enum Play {
    Rock = 1, // Value is score for playing.
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy, Clone, Debug)]
enum PlayResult {
    Win = 6, // Value is score for turn result.
    Draw = 3,
    Loss = 0,
}

impl Play {
    fn new(ch: &str) -> Self {
        match ch {
            "A" | "X" => Play::Rock,
            "B" | "Y" => Play::Paper,
            "C" | "Z" => Play::Scissors,
            _ => panic!("Unknown play string {}", ch),
        }
    }

    fn superior(&self) -> Play {
      match self {
        Play::Rock => Play::Paper,
        Play::Paper => Play::Scissors,
        Play::Scissors => Play::Rock,
      }
    }

    fn inferior(&self) -> Play {
      match self {
        Play::Rock => Play::Scissors,
        Play::Paper => Play::Rock,
        Play::Scissors => Play::Paper,
      }
    }

    fn versus(&self, other: Play) -> PlayResult {
        match self {
            Play::Rock => match other {
                Play::Rock => PlayResult::Draw,
                Play::Paper => PlayResult::Loss,
                Play::Scissors => PlayResult::Win,
            },
            Play::Paper => match other {
                Play::Rock => PlayResult::Win,
                Play::Paper => PlayResult::Draw,
                Play::Scissors => PlayResult::Loss,
            },
            Play::Scissors => match other {
                Play::Rock => PlayResult::Loss,
                Play::Paper => PlayResult::Win,
                Play::Scissors => PlayResult::Draw,
            },
        }
    }
}

#[derive(Debug)]
struct Turn {
    play: Play,
    response: Play,
}

impl Turn {
    fn new(line: &str) -> Self {
        let parts: Vec<&str> = line.split(' ').collect();
        Turn {
            play: Play::new(parts[0]),
            response: Play::new(parts[1]),
        }
    }

    fn score(&self) -> i64 {
        self.response as i64 + self.response.versus(self.play) as i64
    }
}

fn first(input: &String) {
    let mut score_sum: i64 = 0;
    for line in input.lines() {
        let turn = Turn::new(line);
        score_sum += turn.score();
    }
    println!("Total score: {}", score_sum);
}

fn second(input: &String) {
    let mut score_sum: i64 = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let play = Play::new(parts[0]);
        let response = match parts[1] {
          "X" => play.inferior(), // lose
          "Y" => play, // draw
          "Z" => play.superior(), // win
          _ => panic!("Unknown response string {}", parts[1]),
        };
        let turn = Turn {
            play,
            response,
        };
        score_sum += turn.score();
    }
    println!("Total score: {}", score_sum);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    first(&input);
    second(&input);
}
