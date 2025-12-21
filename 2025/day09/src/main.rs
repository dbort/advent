#[derive(Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

fn parse_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().trim().parse::<i64>().unwrap();
            let y = parts.next().unwrap().trim().parse::<i64>().unwrap();
            Point { x, y }
        })
        .collect()
}

fn part1(input: &String) -> i64 {
    let points = parse_points(input);
    let mut max_area = 0;
    for (i, point) in points.iter().enumerate() {
        for j in i + 1..points.len() {
            let other = &points[j];
            let area = ((point.x - other.x).abs() + 1) * ((point.y - other.y).abs() + 1);
            println!("[{}] {:?} x [{}] {:?} => {}", i, point, j, other, area);
            max_area = std::cmp::max(max_area, area);
        }
    }
    max_area
}

fn part2(input: &String) -> i64 {
    let _ = input;
    -1
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename> [<part>]", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    let part = if args.len() >= 3 {
        args[2].parse::<u32>().expect("Failed to parse part number")
    } else {
        3
    };

    println!("Using input from {}", filename);
    let input = std::fs::read_to_string(filename).unwrap();

    if part == 1 || part > 2 {
        println!("Part 1: {}", part1(&input));
    }
    if part == 2 || part > 2 {
        println!("Part 2: {}", part2(&input));
    }
}
