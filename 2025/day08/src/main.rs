#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn distance_to(self, other: Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }
}

fn parse(input: &str) -> Vec<Vec3> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            let x = parts[0].trim().parse::<f64>().unwrap();
            let y = parts[1].trim().parse::<f64>().unwrap();
            let z = parts[2].trim().parse::<f64>().unwrap();
            Vec3::new(x, y, z)
        })
        .collect()
}

#[derive(Debug)]
pub struct Circuits {
    pub box_to_circuit: Vec<usize>,
    pub boxes_in_circuit: Vec<Vec<usize>>,
    num_circuits: usize,
}

impl Circuits {
    pub fn new(num_boxes: usize) -> Self {
        Circuits {
            box_to_circuit: (0..num_boxes).collect(),
            boxes_in_circuit: (0..num_boxes).map(|i| vec![i]).collect(),
            num_circuits: num_boxes,
        }
    }

    pub fn connect_boxes(&mut self, bi1: usize, bi2: usize) {
        let mut c1 = self.box_to_circuit[bi1];
        let mut c2 = self.box_to_circuit[bi2];
        if c1 != c2 {
            self.num_circuits -= 1;
            if c1 > c2 {
                (c1, c2) = (c2, c1);
            }
            for i in self.boxes_in_circuit[c2].iter() {
                self.box_to_circuit[*i] = c1;
            }
            println!("");
            let (vec_head, vec_tail) = self.boxes_in_circuit.split_at_mut(c2);
            let c1_boxes = &mut vec_head[c1];
            let c2_boxes = &vec_tail[0];
            print!(
                "++ [{}] #{} + [{}] #{} ==> ",
                c1,
                c1_boxes.len(),
                c2,
                c2_boxes.len()
            );
            c1_boxes.extend(c2_boxes);
            self.boxes_in_circuit[c2] = vec![];
            println!("#{}", self.boxes_in_circuit[c1].len());
        }
    }
}

/*
- find_circuit(box_index)
- combine_circuits(circuit_index, circuit_index)

- box_to_circuit Vec<usize>
- boxes_in_circuit Vec<Vec<usize>>
*/

fn part1(input: &String) -> i64 {
    let boxes = parse(input);
    // (distance, (box-a-index, box-b-index))
    let distances = {
        let mut tmpdist: Vec<(f64, (usize, usize))> = vec![];
        for (i, bi) in boxes.iter().enumerate() {
            for (j, bj) in boxes.iter().enumerate() {
                if j > i {
                    tmpdist.push((bi.distance_to(*bj), (i, j)));
                }
            }
        }
        tmpdist.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        tmpdist
    };

    let mut circuits = Circuits::new(boxes.len());
    let mut num_to_connect = 1000;
    for (d, (b1i, b2i)) in distances.iter() {
        println!(
            "dist {} [{}]{:?} .. [{}]{:?}",
            d, b1i, boxes[*b1i], b2i, boxes[*b2i]
        );
        circuits.connect_boxes(*b1i, *b2i);
        num_to_connect -= 1;
        if num_to_connect == 0 {
            break;
        }
    }

    let sizes = {
        let mut sizes: Vec<usize> = circuits.boxes_in_circuit.iter().map(|v| v.len()).collect();
        sizes.sort_unstable_by(|a, b| b.cmp(a));
        sizes
    };

    (sizes[0] * sizes[1] * sizes[2]) as i64
}

fn part2(input: &String) -> i64 {
    let boxes = parse(input);
    // (distance, (box-a-index, box-b-index))
    let distances = {
        let mut tmpdist: Vec<(f64, (usize, usize))> = vec![];
        for (i, bi) in boxes.iter().enumerate() {
            for (j, bj) in boxes.iter().enumerate() {
                if j > i {
                    tmpdist.push((bi.distance_to(*bj), (i, j)));
                }
            }
        }
        tmpdist.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        tmpdist
    };

    let mut circuits = Circuits::new(boxes.len());
    for (d, (b1i, b2i)) in distances.iter() {
        println!(
            "dist {} [{}]{:?} .. [{}]{:?}",
            d, b1i, boxes[*b1i], b2i, boxes[*b2i]
        );
        let old_num = circuits.num_circuits;
        circuits.connect_boxes(*b1i, *b2i);
        println!("num_circuits {} => {}", old_num, circuits.num_circuits);
        if circuits.num_circuits == 1 {
            println!("all circuits connected");
            return (boxes[*b1i].x * boxes[*b2i].x) as i64;
        }
    }
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
