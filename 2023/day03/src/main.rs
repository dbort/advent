use std::collections::HashMap;

#[derive(Debug)]
enum Item {
    Number(usize),
    Symbol(char),
}

#[derive(Debug)]
struct PlacedItem {
    item: Item,
    line: usize,
    col: usize,
}

// Returns numbers, symbols
fn scrape_line(line: &str, linenum: usize) -> (Vec<PlacedItem>, Vec<PlacedItem>) {
    let mut numbers: Vec<PlacedItem> = vec![];
    let mut symbols: Vec<PlacedItem> = vec![];
    let mut item_col: usize = 0;
    let mut col: usize = 1;
    let mut value: usize = 0;
    for c in line.chars() {
        if c.is_digit(10) {
            value = value * 10 + c.to_digit(10).unwrap() as usize;
            if item_col == 0 {
                item_col = col;
            }
        } else {
            if value > 0 {
                numbers.push(PlacedItem {
                    item: Item::Number(value),
                    line: linenum,
                    col: item_col,
                });
                // println!("{:?}", numbers.last().unwrap());
                item_col = 0;
                value = 0;
            }
            if c != '.' {
                symbols.push(PlacedItem {
                    item: Item::Symbol(c),
                    line: linenum,
                    col: col,
                });
                // println!("{:?}", symbols.last().unwrap());
            }
        }
        col += 1;
    }
    (numbers, symbols)
}

fn first(input: &String) {
    let mut numbers: Vec<PlacedItem> = vec![];
    let mut symbols: HashMap<(usize, usize), char> = HashMap::new();
    let mut linenum: usize = 1;
    for line in input.lines() {
        let (mut numbers_1, symbols_1) = scrape_line(line, linenum);
        numbers.append(&mut numbers_1);
        for s in symbols_1 {
            match s.item {
                Item::Symbol(c) => {
                    symbols.insert((s.line, s.col), c);
                }
                _ => unreachable!(),
            }
        }
        linenum += 1;
    }

    let mut sum: usize = 0;
    for n in &numbers {
        let value = match n.item {
            Item::Number(n) => n,
            _ => unreachable!(),
        };
        let col_end = n.col + value.to_string().len() - 1;
        let mut adjacent = false;
        for col in (n.col - 1)..=(col_end + 1) {
            for line in (n.line - 1)..=(n.line + 1) {
                if symbols.contains_key(&(line, col)) {
                    adjacent = true;
                    break;
                }
            }
        }
        if adjacent {
            // println!("{} marked", value);
            sum += value;
        } else {
            // println!("{} not marked", value);
        }
    }
    println!("First total: {}", sum);
}

fn second(input: &String) {}

fn main() {
    let input = std::fs::read_to_string("sample-input.txt").unwrap();
    first(&input);
    second(&input);
}
