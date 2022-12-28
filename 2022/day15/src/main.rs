extern crate gcollections;
extern crate interval;
use crate::interval::interval_set::*;
use gcollections::ops::*;
use regex::Regex;
use std::cmp;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

    fn x_range_for_y(&self, y: isize) -> (isize, isize) {
        let ydist = isize::abs(self.location.y - y);
        let diameter = self.diameter();
        if ydist <= diameter {
            let halfw = diameter - ydist;
            (self.location.x - halfw, self.location.x + halfw)
        } else {
            (0, 0)
        }
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
        println!(
            "  x range for {target_y}: {:?}",
            sensor.x_range_for_y(target_y)
        );
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
        let (x0, x1) = sensor.x_range_for_y(target_y);
        if (x0, x1) != (0, 0) {
            minx = cmp::min(x0, minx);
            maxx = cmp::max(x1, maxx);
        }
    }
    println!(
        "x range for target y {target_y}: {minx}..{maxx} ({})",
        maxx - minx
    );
    let mut covered: usize = 0;
    let mut uncovered: usize = 0;
    for x in minx..=maxx {
        let p = Point { x, y: target_y };
        let mut is_covered = false;
        for sensor in &covering_sensors {
            if sensor.covers(p) {
                is_covered = true;
            }
            if sensor.beacon == p {
                // In case it's possible for only one sensor to know
                // that this beacon exists.
                is_covered = false;
                break;
            }
        }
        if is_covered {
            covered += 1;
        } else {
            uncovered += 1;
        }
    }
    println!("Covered: {covered}, uncovered: {uncovered}");
}

fn second(input: &String, max_loc: isize) {
    let sensors: Vec<Sensor> = input.lines().map(|s| Sensor::new(s)).collect();
    for y in 0..=max_loc {
        let mut intervals = vec![(0 as isize, 0 as isize)].to_interval_set();
        for sensor in &sensors {
            let (x0, x1) = sensor.x_range_for_y(y);
            if (x0, x1) == (0, 0) {
                continue;
            }
            intervals = intervals.union(&vec![(x0, x1)].to_interval_set());
        }
        intervals = intervals.intersection(&vec![(0, max_loc)].to_interval_set());
        if intervals.size() != max_loc as usize + 1 {
            println!("y {y}: {:?}", intervals);
            break;
            // y 3017867: { lb: 0, ub: 3068580 }, { lb: 3068582, ub: 4000000 }], size: 4000000
            // >>> 3068581 * 4000000 + 3017867
            // 12274327017867

            // sample
            // y 11: { lb: 0, ub: 13 }, { lb: 15, ub: 20 }], size: 20
        }
    }
}

fn main() {
    const USE_SAMPLE: bool = true;
    let (input_file, target_y, max_loc) = if USE_SAMPLE {
        ("sample-input.txt", 10, 20)
    } else {
        ("input.txt", 2000000, 4000000)
    };
    let input = std::fs::read_to_string(input_file).unwrap();
    first(&input, target_y);
    second(&input, max_loc);
}
