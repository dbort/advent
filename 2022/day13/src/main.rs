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
        0
    }
}

fn first(lines: &Vec<&str>) {
    let mut i: usize = 0;
    while i < lines.len() {
        assert!(i + 1 < lines.len());
        let left = Item::parse(&mut Stream::new(lines[i]));
        println!("Left {i}: {:?}", left);
        let right = Item::parse(&mut Stream::new(lines[i + 1]));
        println!("Right {i}: {:?}\n", right);
        i += 3;
    }
}

fn second(lines: &Vec<&str>) {}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    first(&lines);
    second(&lines);
}
