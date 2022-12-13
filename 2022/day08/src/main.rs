#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<usize>>,
}

#[derive(Debug)]
struct Visibility {
    trees: Vec<Vec<usize>>,
}

impl Visibility {
    fn sum(&self) -> usize {
        let mut sum: usize = 0;
        for row in self.trees.iter() {
            sum += row.iter().sum::<usize>();
        }
        return sum;
    }
}

impl Forest {
    fn new(spec: &String) -> Self {
        let mut trees: Vec<Vec<usize>> = Vec::new();
        for line in spec.lines() {
            let row: Vec<usize> = line
                .chars()
                .map(|c| String::from(c).parse::<usize>().unwrap())
                .collect();
            trees.push(row)
        }
        Forest { trees }
    }

    fn visibility(&self) -> Visibility {
        let mut trees: Vec<Vec<usize>> = Vec::new();
        let rows = self.trees.len();
        let cols = self.trees[0].len();
        println!("rows {rows} x cols {cols}");
        for row in 0..rows {
            let mut row_vis: Vec<usize> = Vec::new();
            for col in 0..cols {
                let height = self.trees[row][col];
                // Left
                let mut vis = true;
                for i in 0..col {
                    if self.trees[row][i] >= height {
                        vis = false;
                        break;
                    }
                }
                if vis {
                    row_vis.push(1);
                    continue;
                }
                // Right
                vis = true;
                for i in (col + 1)..cols {
                    if self.trees[row][i] >= height {
                        vis = false;
                        break;
                    }
                }
                if vis {
                    row_vis.push(1);
                    continue;
                }
                // Top
                vis = true;
                for i in 0..row {
                    if self.trees[i][col] >= height {
                        vis = false;
                        break;
                    }
                }
                if vis {
                    row_vis.push(1);
                    continue;
                }
                // Bottom
                vis = true;
                for i in (row + 1)..rows {
                    if self.trees[i][col] >= height {
                        vis = false;
                        break;
                    }
                }
                if vis {
                    row_vis.push(1);
                    continue;
                }
                row_vis.push(0);
            }
            trees.push(row_vis);
        }
        Visibility { trees }
    }

    fn scenic(&self) -> Vec<usize> {
        let mut scores: Vec<usize> = Vec::new();
        let rows = self.trees.len();
        let cols = self.trees[0].len();
        println!("rows {rows} x cols {cols}");
        for row in 0..rows {
            for col in 0..cols {
                let height = self.trees[row][col];
                let mut count_score = 1;
                // Left
                let mut count = 0;
                for i in (0..col).rev() {
                    count += 1;
                    if self.trees[row][i] >= height {
                        break;
                    }
                }
                count_score *= count;
                count = 0;
                // Right
                for i in (col + 1)..cols {
                    count += 1;
                    if self.trees[row][i] >= height {
                        break;
                    }
                }
                count_score *= count;
                count = 0;
                // Top
                for i in (0..row).rev() {
                    count += 1;
                    if self.trees[i][col] >= height {
                        break;
                    }
                }
                count_score *= count;
                count = 0;
                // Bottom
                for i in (row + 1)..rows {
                    count += 1;
                    if self.trees[i][col] >= height {
                        break;
                    }
                }
                count_score *= count;
                scores.push(count_score);
            }
        }
        scores.sort_unstable_by(|a, b| b.cmp(a));
        scores
    }
}

fn first(input: &String) {
    let forest = Forest::new(input);
    println!("{:?}", forest);
    let visibility = forest.visibility();
    println!("{:?}", visibility);
    println!("visible trees: {}", visibility.sum());
}

fn second(input: &String) {
    let forest = Forest::new(input);
    println!("{:?}", forest);
    let scores = forest.scenic();
    println!("top score: {}", scores[0]);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    first(&input);
    second(&input);
}
