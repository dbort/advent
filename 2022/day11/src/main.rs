use regex::Regex;

#[derive(Debug)]
enum Op {
    Add(usize),
    Mul(usize),
    Square,
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<usize>,
    op: Op,
    test_div: usize,
    true_dest: usize,
    false_dest: usize,
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
        let items: Vec<usize> = item_list
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
            items,
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
        }
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

fn first(input: &String) {
    let monkeys = read_monkeys(input);
    println!("Monkeys: {:?}", monkeys);
}

fn second(input: &String) {}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    first(&input);
    second(&input);
}
