use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn movehead(&self, dir: &str) -> Point {
        let mut xinc = 0;
        let mut yinc = 0;
        match dir {
            "U" => yinc = 1,
            "D" => yinc = -1,
            "L" => xinc = -1,
            "R" => xinc = 1,
            _ => panic!("Unexpected direction '{}'", dir),
        }
        Point {
            x: self.x + xinc,
            y: self.y + yinc,
        }
    }

    fn touches(&self, other: &Point) -> bool {
        i64::abs(self.x - other.x) <= 1 && i64::abs(self.y - other.y) <= 1
    }

    fn follow(&self, other: &Point) -> Point {
        if self.touches(other) {
            println!(">>> Still touching");
            return *self;
        }
        let newy: i64 = if self.y < other.y {
            self.y + 1
        } else if self.y > other.y {
            self.y - 1
        } else {
            self.y
        };
        let newx: i64 = if self.x < other.x {
            self.x + 1
        } else if self.x > other.x {
            self.x - 1
        } else {
            self.x
        };
        Point { x: newx, y: newy }
    }
}

fn simulate(input: &String) -> HashSet<Point> {
    let mut points: HashSet<Point> = HashSet::new();
    let mut head = Point { x: 0, y: 0 };
    let mut tail = head;
    points.insert(tail);
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        assert!(parts.len() == 2);
        let mag = parts[1].parse::<i64>().unwrap();
        for i in 0..mag {
            print!("Move head {:?} {}", head, parts[0]);
            head = head.movehead(parts[0]);
            println!(" -> {:?}", head);

            print!("Move tail {:?}", tail);
            tail = tail.follow(&head);
            println!(" -> {:?}", tail);
            points.insert(tail);
        }
    }
    points
}

fn first(input: &String) {
    let points = simulate(input);
    println!("num tailpos {}", points.len());
}

fn simulate2(input: &String, num_knots: usize) -> HashSet<Point> {
    let mut points: HashSet<Point> = HashSet::new();

    let mut knots = vec![Point { x: 0, y: 0 }; num_knots];
    points.insert(knots[num_knots - 1]);

    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        assert!(parts.len() == 2);
        let mag = parts[1].parse::<i64>().unwrap();
        for i in 0..mag {
            print!("Move head {:?} {}", knots[0], parts[0]);
            knots[0] = knots[0].movehead(parts[0]);
            println!(" -> {:?}", knots[0]);

            for k in 1..num_knots {
                print!("Move [{k}] {:?}", knots[k]);
                knots[k] = knots[k].follow(&knots[k - 1]);
                println!(" -> {:?}", knots[k]);
            }
            points.insert(knots[num_knots - 1]);
        }
    }
    points
}

fn second(input: &String) {
    let points = simulate2(input, 10);
    println!("pt2 num tailpos {}", points.len());
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    first(&input);
    second(&input);
}
