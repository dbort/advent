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

    fn cmp(&self, other: &Item) -> i8 {
        match (self, other) {
            (Item::Value(s), Item::Value(o)) => {
                if s < o {
                    -1
                } else if s > o {
                    1
                } else {
                    0
                }
            }
            (Item::List(s), Item::List(o)) => {
                let mut cmp = 0;
                for i in 0..s.len() {
                    if i >= o.len() {
                        cmp = 1;
                        break;
                    }
                    cmp = s[i].cmp(&o[i]);
                    if cmp != 0 {
                        break;
                    }
                }
                if cmp == 0 {
                    Item::Value(s.len()).cmp(&Item::Value(o.len()))
                } else {
                    cmp
                }
            }
            (Item::Value(s), Item::List(o)) => Item::List(vec![Item::Value(*s)]).cmp(other),
            (Item::List(s), Item::Value(o)) => self.cmp(&Item::List(vec![Item::Value(*o)])),
        }
    }
}

fn first(lines: &Vec<&str>) {
    let mut sum = 0;
    let mut i: usize = 0;
    while i < lines.len() {
        assert!(i + 1 < lines.len());
        let left = Item::parse(&mut Stream::new(lines[i]));
        println!("Left {i}: {:?}", left);
        let right = Item::parse(&mut Stream::new(lines[i + 1]));
        println!("Right {i}: {:?}", right);
        let cmp = left.cmp(&right);
        assert!(cmp != 0);
        if cmp < 0 {
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

fn second(lines: &Vec<&str>) {}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    first(&lines);
    second(&lines);
}
