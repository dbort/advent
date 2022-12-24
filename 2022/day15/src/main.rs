use regex::Regex;
use std::cmp;
use std::fmt;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Sensor {
    location: Point,
    beacon: Point,
}

fn str_capture(captures: &regex::Captures, name: &str) -> String {
    String::from(captures.name(name).unwrap().as_str())
}

fn num_capture(captures: &regex::Captures, name: &str) -> isize {
    str_capture(captures, name).parse::<isize>().unwrap()
}

impl Sensor {
    fn new(spec: &str) -> Self {
        let re = Regex::new(concat!(
            r"Sensor at ",
            "x=(?P<sensor_x>-?[[:digit:]]+), ",
            "y=(?P<sensor_y>-?[[:digit:]]+): ",
            "closest beacon is at ",
            "x=(?P<beacon_x>-?[[:digit:]]+), ",
            "y=(?P<beacon_y>-?[[:digit:]]+)",
        ))
        .unwrap();
        let cap = re.captures(spec).unwrap();
        Sensor {
            location: Point {
                x: num_capture(&cap, "sensor_x"),
                y: num_capture(&cap, "sensor_y"),
            },
            beacon: Point {
                x: num_capture(&cap, "beacon_x"),
                y: num_capture(&cap, "beacon_y"),
            },
        }
    }

    fn distance_to(&self, point: Point) -> isize {
        isize::abs(self.location.x - point.x) + isize::abs(self.location.y - point.y)
    }

    fn diameter(&self) -> isize {
        self.distance_to(self.beacon)
    }

    fn covers(&self, point: Point) -> bool {
        self.distance_to(point) <= self.diameter()
    }

    fn covers_y(&self, y: isize) -> bool {
        self.covers(Point {
            x: self.location.x,
            y,
        })
    }

    fn x_range(&self) -> (isize, isize) {
        (
            self.location.x - self.diameter(),
            self.location.x + self.diameter(),
        )
    }
}

impl fmt::Display for Sensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "loc={}, bcn={}, dia={}",
            self.location,
            self.beacon,
            self.diameter()
        )
    }
}

fn first(input: &String, target_y: isize) {
    let sensors: Vec<Sensor> = input.lines().map(|s| Sensor::new(s)).collect();
    for sensor in &sensors {
        println!("{}", sensor);
        println!("  covers y {target_y}: {}", sensor.covers_y(target_y));
        println!("  x range {:?}", sensor.x_range());
    }
    let covering_sensors: Vec<&Sensor> = sensors.iter().filter(|s| s.covers_y(target_y)).collect();
    println!(
        "{}/{} sensors cover {target_y}",
        covering_sensors.len(),
        sensors.len()
    );
    let mut minx = isize::MAX;
    let mut maxx = isize::MIN;
    for sensor in &covering_sensors {
        let (x, y) = sensor.x_range();
        minx = cmp::min(x, minx);
        maxx = cmp::max(x, maxx);
    }
    println!(
        "x range for target y {target_y}: {minx}..{maxx} ({})",
        maxx - minx
    )
}

fn second(input: &String) {}

fn main() {
    const USE_SAMPLE: bool = false;
    let (input_file, target_y) = if USE_SAMPLE {
        ("sample-input.txt", 10)
    } else {
        ("input.txt", 2000000)
    };
    let input = std::fs::read_to_string(input_file).unwrap();
    first(&input, target_y);
    second(&input);
}
