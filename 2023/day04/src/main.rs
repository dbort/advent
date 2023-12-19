use std::collections::{HashMap, HashSet};

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
    let mut cards: HashMap<usize, Card> = HashMap::new();
    for line in input.lines() {
        let card = Card::parse(line);
        cards.insert(card.number, card);
    }
    let mut won_cards: Vec<&Card> = vec![];
    for (_, card) in &cards {
        won_cards.push(card);
        let score = {
            let nw = card.num_winners();
            if nw > 0 {
                usize::pow(2, nw as u32 - 1)
            } else {
                0
            }
        };
        print!("card {} won {}: ", card.number, score);
        for i in card.number + 1..=card.number + score {
            if cards.contains_key(&i) {
                print!("{}, ", i);
                won_cards.push(cards.get(&i).unwrap());
            }
        }
        print!("\n");
    }
    // need to play all cards first, counting winners, then score??
    won_cards.len() as i64
}

fn main() {
    let input = std::fs::read_to_string("sample-input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
