fn parse_grid(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => true,
                    '.' => false,
                    _ => panic!("Invalid character encountered: {}", c),
                })
                .collect()
        })
        .collect()
}

fn part1(input: &String) -> i64 {
    let grid = parse_grid(input);
    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    for y in 0..rows {
        for x in 0..cols {
            let mut neighbors = 0;
            print!("(y {}, x {}) => ", y, x);
            if !grid[y][x] {
                println!("x");
                continue;
            }
            for i in if y > 0 { y - 1 } else { y }..=if y < rows - 1 { y + 1 } else { y } {
                for j in if x > 0 { x - 1 } else { x }..=if x < cols - 1 { x + 1 } else { x } {
                    print!("[i {}, j {}]", i, j);
                    if (i, j) != (y, x) {
                        if grid[i][j] {
                            neighbors += 1;
                            print!("@, ");
                        } else {
                            print!("_, ");
                        }
                    } else {
                        print!("., ");
                    }
                }
            }
            println!("");
            if neighbors < 4 {
                count += 1;
            }
        }
    }
    count
}

fn count_and_remove(grid: &mut Vec<Vec<bool>>) -> i64 {
    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    for y in 0..rows {
        for x in 0..cols {
            let mut neighbors = 0;
            print!("(y {}, x {}) => ", y, x);
            if !grid[y][x] {
                println!("x");
                continue;
            }
            for i in if y > 0 { y - 1 } else { y }..=if y < rows - 1 { y + 1 } else { y } {
                for j in if x > 0 { x - 1 } else { x }..=if x < cols - 1 { x + 1 } else { x } {
                    print!("[i {}, j {}]", i, j);
                    if (i, j) != (y, x) {
                        if grid[i][j] {
                            neighbors += 1;
                            print!("@, ");
                        } else {
                            print!("_, ");
                        }
                    } else {
                        print!("., ");
                    }
                }
            }
            println!("");
            if neighbors < 4 {
                count += 1;
                grid[y][x] = false;
            }
        }
    }
    count
}

fn part2(input: &String) -> i64 {
    let nonmut_grid = parse_grid(input);

    let mut grid: Vec<Vec<bool>> = nonmut_grid.iter().map(|i| i.clone()).collect();
    let mut count = 0;
    while true {
        let new_count = count_and_remove(&mut grid);
        if new_count == 0 {
            break;
        }
        count += new_count;
    }
    count
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
