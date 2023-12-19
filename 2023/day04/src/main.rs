use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    number: usize,
    winners: HashSet<usize>,
    have: HashSet<usize>,
}

impl Card {
    fn parse(line: &str) -> Self {
        let (card, (winners, have)) = line
            .strip_prefix("Card ")
            .and_then(|s| Some(s.trim()))
            .and_then(|s| s.split_once(':'))
            .and_then(|(card, rest)| Some((card, rest.split_once('|').unwrap())))
            .unwrap();
        Self {
            number: card.parse::<usize>().unwrap(),
            winners: winners
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
            have: have
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
        }
    }

    fn num_winners(&self) -> usize {
        let mut num: usize = 0;
        for h in &self.have {
            // println!("is {} in {:?}?", h, self.winners);
            if self.winners.contains(&h) {
                // println!(">> yep");
                num += 1;
            }
        }
        num
    }
}

fn part1(input: &String) -> i64 {
    let mut sum: i64 = 0;
    for line in input.lines() {
        let card = Card::parse(line);
        let num_winners = card.num_winners();
        println!("{} winners in {:?}", num_winners, card);
        if num_winners > 0 {
            sum += i64::pow(2, num_winners as u32 - 1)
        }
    }
    sum
}

fn part2(input: &String) -> i64 {
    -1
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
