use itertools::Itertools; // for sorted()
use std::cmp::Ordering;

#[derive(Debug)]
enum Item {
    Value(usize),
    List(Vec<Item>),
}

struct Stream<'a> {
    chars: &'a [u8],
    index: usize,
}

impl<'a> Stream<'a> {
    fn new(spec: &'a str) -> Self {
        Stream {
            chars: spec.as_bytes(),
            index: 0,
        }
    }

    fn next(&mut self) -> char {
        self.index += 1;
        self.chars[self.index - 1] as char
    }

    fn peek(&self) -> char {
        self.chars[self.index] as char
    }

    fn empty(&self) -> bool {
        self.index >= self.chars.len()
    }
}

impl Item {
    fn parse(stream: &mut Stream) -> Self {
        assert!(!stream.empty());
        match stream.next() {
            '[' => {
                let mut items: Vec<Item> = Vec::new();
                while stream.peek() != ']' {
                    items.push(Item::parse(stream));
                    if stream.peek() == ',' {
                        stream.next();
                    }
                }
                assert!(stream.next() == ']');
                Item::List(items)
            }
            v @ '0'..='9' => {
                let mut val = v as usize - '0' as usize;
                while stream.peek().is_digit(10) {
                    let v = stream.next();
                    val = val * 10 + (v as usize - '0' as usize);
                }
                Item::Value(val)
            }
            _ => panic!("Unexpected initial char {}", stream.peek()),
        }
    }

    fn cmp(&self, other: &Item) -> Ordering {
        match (self, other) {
            (Item::Value(s), Item::Value(o)) => {
                if s < o {
                    Ordering::Less
                } else if s > o {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            (Item::List(s), Item::List(o)) => {
                let mut cmp = Ordering::Equal;
                for i in 0..s.len() {
                    if i >= o.len() {
                        cmp = Ordering::Greater;
                        break;
                    }
                    cmp = s[i].cmp(&o[i]);
                    if cmp != Ordering::Equal {
                        break;
                    }
                }
                if cmp == Ordering::Equal {
                    Item::Value(s.len()).cmp(&Item::Value(o.len()))
                } else {
                    cmp
                }
            }
            (Item::Value(s), Item::List(_)) => Item::List(vec![Item::Value(*s)]).cmp(other),
            (Item::List(_), Item::Value(o)) => self.cmp(&Item::List(vec![Item::Value(*o)])),
        }
    }
}

fn first(input: &String) {
    let mut sum = 0;
    let mut i: usize = 0;
    let lines: Vec<&str> = input.lines().collect();
    while i < lines.len() {
        assert!(i + 1 < lines.len());
        let left = Item::parse(&mut Stream::new(lines[i]));
        // println!("Left {i}: {:?}", left);
        let right = Item::parse(&mut Stream::new(lines[i + 1]));
        // println!("Right {i}: {:?}", right);
        let cmp = left.cmp(&right);
        assert!(cmp != Ordering::Equal);
        if cmp == Ordering::Less {
            // In the right order
            let pair_index = i / 3 + 1;
            sum += pair_index;
            println!("Pair {pair_index} in the right order");
        }
        println!("");
        i += 3;
    }
    println!("\nSum {sum}");
}

fn second(input: &String) {
    let input2 = input.to_owned() + "[[2]]\n[[6]]\n";
    let packets: Vec<Item> = input2
        .lines()
        .filter(|s| s.len() > 0)
        .map(|s| Item::parse(&mut Stream::new(s)))
        .sorted_by(|a, b| a.cmp(&b))
        .collect();
    let packet2 = Item::parse(&mut Stream::new("[[2]]"));
    let packet6 = Item::parse(&mut Stream::new("[[6]]"));
    let mut decoder_key: usize = 1;
    for (i, packet) in packets.iter().enumerate() {
        if packet.cmp(&packet2) == Ordering::Equal || packet.cmp(&packet6) == Ordering::Equal {
            decoder_key *= i + 1;
        }
    }
    println!("\nDecoder key: {decoder_key}");
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    first(&input);
    second(&input);
}
