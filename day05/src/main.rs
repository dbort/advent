#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn parse_line(line: &str) -> (Point, Point) {
    let re = regex::Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let caps = re.captures(line).unwrap();

    (
        Point {
            x: caps.get(1).unwrap().as_str().to_string().parse::<usize>().unwrap(),
            y: caps.get(2).unwrap().as_str().to_string().parse::<usize>().unwrap(),
        },
        Point {
            x: caps.get(3).unwrap().as_str().to_string().parse::<usize>().unwrap(),
            y: caps.get(4).unwrap().as_str().to_string().parse::<usize>().unwrap(),
        },
    )
}

fn is_on_axes(p1: &Point, p2: &Point) -> bool {
    p1.x == p2.x || p1.y == p2.y
}

struct Image(Vec<Vec<u8>>);

impl Image {
    fn new(width: usize, height: usize) -> Self {
        Self(vec![vec![0; width]; height])
    }

    fn draw(&mut self, p1: &Point, p2: &Point) {
	// let dx = p2.x as isize - p1.x as isize;
	// let dy = p2.y as isize - p1.y as isize;
	// let mut d = 2*dy - dx;
	// let mut y = p1.y;

	// for x in p1.x..=p2.x {
        //     self.0[x][y] += 1;
	//     if d > 0 {
	// 	y = y + 1;
	// 	d = d - 2*dx;
        //     }
	//     d = d + 2*dy
        // }
        if p1.x == p2.x {
            let (min_y, max_y) = if p1.y < p2.y { (p1.y, p2.y) } else { (p2.y, p1.y) };
            for y in min_y..=max_y {
                self.0[y][p1.x] += 1;
            }
            return
        }
        if p1.y == p2.y {
            let (min_x, max_x) = if p1.x < p2.x { (p1.x, p2.x) } else { (p2.x, p1.x) };
            for x in min_x..=max_x {
                self.0[p1.y][x] += 1;
            }
            return
        }
        println!("Skipped line {:?},{:?}", p1, p2);
    }

    fn count_ge(&self, threshold: u8) -> usize {
        let mut ret = 0;
        for row in &self.0 {
            for col in row {
                if *col >= threshold {
                    ret += 1;
                }
            }
        }
        ret
    }

    fn render(&self, out_file: &str) {
        use image::ImageBuffer;

	let img = ImageBuffer::from_fn(self.0.len() as u32, self.0[0].len() as u32, |x, y| {
	    image::Luma([self.0[x as usize][y as usize] as u8])
	});
        img.save(out_file).unwrap();
    }
}

fn first(input: &String) {
    let mut img = Image::new(1000, 1000);

    for line in input.lines() {
        let (p1, p2) = parse_line(line);
        // if !is_on_axes(&p1, &p2) {
        //     continue;
        // }
        img.draw(&p1, &p2);
    }
    img.render("/tmp/img.png");
    println!("Count: {}", img.count_ge(2));
    // for row in img.0 {
    //     println!("{:?}", row);
    // }
}

fn second(_input: &String) {
}

fn main() {
    let input = common::input::load_file("input.txt");
    first(&input);
    second(&input);
}
