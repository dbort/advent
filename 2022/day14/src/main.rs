use std::cmp;

// 528 is the largest value in the input
const MAX_DIM: usize = 550;
const MAP_HEIGHT: usize = MAX_DIM;
const MAP_WIDTH: usize = MAX_DIM;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(spec: &str) -> Self {
        let parts: Vec<&str> = spec.split(',').collect();
        assert!(parts.len() == 2);
        Point {
            x: parts[0].parse::<usize>().unwrap(),
            y: parts[1].parse::<usize>().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Polyline {
    points: Vec<Point>,
}

impl Polyline {
    fn new(spec: &str) -> Self {
        Polyline {
            points: spec.split(" -> ").map(|s| Point::new(s)).collect(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Material {
    Air,
    Rock,
    Sand,
}

impl Material {
    fn as_char(&self) -> char {
        match self {
            Material::Air => '.',
            Material::Rock => '#',
            Material::Sand => 'o',
        }
    }
}

struct Map {
    entry: Point,
    contents: [Material; MAP_HEIGHT * MAP_WIDTH],
}

impl Map {
    fn print_window(&self, min_row: usize, max_row: usize, min_col: usize, max_col: usize) {
        for row in min_row..max_row {
            print!("{row:>3}: ");
            for col in min_col..max_col {
                let c = if self.entry.x == col && self.entry.y == row {
                    '+'
                } else {
                    self.contents[row * MAP_WIDTH + col].as_char()
                };
                print!("{c}");
            }
            println!("");
        }
        println!("---");
    }

    fn print_around(&self, row: usize, col: usize) {
        const ROW_PAD: usize = 10;
        const COL_PAD: usize = 20;
        let row = cmp::max(row, ROW_PAD);
        let row = cmp::min(row, MAP_HEIGHT - ROW_PAD);
        let col = cmp::max(col, COL_PAD);
        let col = cmp::min(col, MAP_HEIGHT - COL_PAD);
        self.print_window(row - ROW_PAD, row + ROW_PAD, col - COL_PAD, col + COL_PAD);
    }

    fn print(&self) {
        self.print_window(0, MAP_HEIGHT, 0, MAP_WIDTH);
    }

    fn populate(&mut self, points: &Vec<Point>, material: Material) {
        for (i, point) in points.iter().enumerate() {
            if (i > 0) {
                let prev = points[i - 1];
                let miny = cmp::min(prev.y, point.y);
                let maxy = cmp::max(prev.y, point.y);
                let minx = cmp::min(prev.x, point.x);
                let maxx = cmp::max(prev.x, point.x);
                for y in miny..=maxy {
                    for x in minx..=maxx {
                        self.contents[y * MAP_WIDTH + x] = material;
                    }
                }
            }
        }
    }
}

fn first(input: &String) {
    let rocks: Vec<Polyline> = input.lines().map(|s| Polyline::new(s)).collect();

    let mut map = Map {
        entry: Point { x: 500, y: 0 },
        contents: [Material::Air; MAP_HEIGHT * MAP_WIDTH],
    };

    for rock in rocks {
        map.populate(&rock.points, Material::Rock);
    }

    map.print_around(map.entry.y, map.entry.x);
}

fn second(input: &String) {}

fn main() {
    let input = std::fs::read_to_string("sample-input.txt").unwrap();
    first(&input);
    second(&input);
}