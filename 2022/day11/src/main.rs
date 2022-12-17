use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug)]
enum Op {
    Add(usize),
    Mul(usize),
    Square,
}

impl Op {
    fn execute(&self, old: usize) -> usize {
        match self {
            Op::Add(n) => old + n,
            Op::Mul(n) => old * n,
            Op::Square => old * old,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    op: Op,
    test_div: usize,
    true_dest: usize,
    false_dest: usize,
    items: VecDeque<usize>,
    inspections: usize,
}

fn str_capture(captures: &regex::Captures, name: &str) -> String {
    String::from(captures.name(name).unwrap().as_str())
}

fn num_capture(captures: &regex::Captures, name: &str) -> usize {
    str_capture(captures, name).parse::<usize>().unwrap()
}

impl Monkey {
    fn new(spec: Vec<&str>) -> Self {
        assert!(spec.len() == 6);
        let re_id = Regex::new(r"Monkey (?P<id>[[:digit:]]+):").unwrap();
        let id = num_capture(&re_id.captures(spec[0]).unwrap(), "id");

        let re_items = Regex::new(r"items: (?P<items>[[:digit:], ]+)").unwrap();
        let item_list = str_capture(&re_items.captures(spec[1]).unwrap(), "items");
        let items: VecDeque<usize> = item_list
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let re_op = Regex::new(r"new = old (?P<op>[+*]) (?P<val>\S+)").unwrap();
        let op_cap = re_op.captures(spec[2]).unwrap();
        let op_str = str_capture(&op_cap, "op");
        let val_str = str_capture(&op_cap, "val");
        println!("{id}: {op_str} {val_str}");

        let re_test_div = Regex::new(r" by (?P<test_div>[[:digit:]]+)").unwrap();
        let test_div = num_capture(&re_test_div.captures(spec[3]).unwrap(), "test_div");

        let re_dest = Regex::new(r" throw to monkey (?P<id>[[:digit:]]+)").unwrap();
        let true_dest = num_capture(&re_dest.captures(spec[4]).unwrap(), "id");
        let false_dest = num_capture(&re_dest.captures(spec[5]).unwrap(), "id");
        Monkey {
            id,
            op: match op_str.as_str() {
                "+" => Op::Add(val_str.parse::<usize>().unwrap()),
                "*" => match val_str.as_str() {
                    "old" => Op::Square,
                    _ => Op::Mul(val_str.parse::<usize>().unwrap()),
                },
                _ => panic!("Unhandled op {op_str}"),
            },
            test_div,
            true_dest,
            false_dest,
            items,
            inspections: 0,
        }
    }

    fn tostr(&self) -> String {
        format!(
            "Monkey {} [{}]: {:?}",
            self.id, self.inspections, self.items
        )
    }

    fn inspect_one_item(&mut self) -> (usize, usize) {
        let item = self.items.pop_front().unwrap();
        let item = self.op.execute(item);
        let item = item / 3;

        let dest = match item % self.test_div {
            0 => self.true_dest,
            _ => self.false_dest,
        };

        self.inspections += 1;
        (item, dest)
    }
}

fn read_monkeys(input: &String) -> Vec<Monkey> {
    let mut record: Vec<&str> = Vec::new();
    let mut monkeys: Vec<Monkey> = Vec::new();
    for line in input.lines() {
        if line == "" {
            monkeys.push(Monkey::new(record));
            record = Vec::new();
        } else {
            record.push(line);
        }
    }
    monkeys.push(Monkey::new(record));
    monkeys
}

fn run_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        'monkey: loop {
            let x: Option<(usize, usize)> = if monkeys[i].items.is_empty() {
                None
            } else {
                Some(monkeys[i].inspect_one_item())
            };
            match x {
                Some((item, dest)) => {
                    assert!(dest != i);
                    monkeys[dest].items.push_back(item);
                }
                _ => break 'monkey,
            };
        }
    }
}

fn first(input: &String) {
    let mut monkeys = read_monkeys(input);
    println!("Monkeys: {:?}", monkeys);
    let num_rounds = 20;
    for _ in 1..=num_rounds {
        run_round(&mut monkeys);
    }
    println!("After {num_rounds} rounds:");
    for monkey in &monkeys {
        println!("  {}", monkey.tostr());
    }
    let mut scores: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
    scores.sort_unstable_by(|a, b| b.cmp(a));
    println!("  Scores: {:?}", scores);
    assert!(scores.len() >= 2);
    println!("  Sum: {}", scores[0] * scores[1]);
}

fn second(input: &String) {}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    first(&input);
    second(&input);
}
