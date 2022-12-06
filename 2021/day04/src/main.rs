#[derive(Debug)]
struct Board {
   numbers: Vec<Vec<i8>>,
}

impl Board {
    fn new(lines: Vec<String>) -> Self {
        let mut numbers: Vec<Vec<i8>> = vec![];
        for line in lines {
            let row: Vec<i8> = line
                .split(' ')
                .filter(|s| s.len() > 0)
                .map(|s| s.parse::<i8>().unwrap())
                .collect();
            numbers.push(row);
        }
        Board {
            numbers,
        }
    }

    fn rows_win(numbers: &Vec<Vec<i8>>) -> bool {
        for row in numbers {
            let mut hits = 0;
            for col in row {
                if *col == -1 {
                    hits += 1;
                }
            }
            if hits == row.len() {
                return true;
            }
        }
        false
    }

    fn rotate(numbers: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
        let mut ret = vec![vec![0 as i8; numbers.len()]; numbers[0].len()];

        for (i, row) in numbers.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                ret[j][i] = *col;
            }
        }
        ret
    }

    fn mark(numbers: &mut Vec<Vec<i8>>, ball: u8) {
        for i in 0..numbers.len() {
            for j in 0..numbers[0].len() {
                if numbers[i][j] == ball as i8 {
                  numbers[i][j] = -1;
                }
            }
        }
    }

    fn calc_score(numbers: &Vec<Vec<i8>>, ball: u8) -> u64 {
        let mut total: u64 = 0;
        for (i, row) in numbers.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                if numbers[i][j] >= 0 {
                    total += numbers[i][j] as u64;
                }
            }
        }
        total * ball as u64
    }

    fn score(&self, balls: &Vec<u8>) -> (usize, u64) {
        let mut numbers = self.numbers.clone();
        for (i, ball) in balls.iter().enumerate() {
            Self::mark(&mut numbers, *ball);
            if Self::rows_win(&numbers) || Self::rows_win(&Self::rotate(&numbers)) {
                return (i, Self::calc_score(&numbers, *ball));
            }
        }
        (balls.len(), 0)
    }
}

fn first(input: &String) {
    enum State {
        Balls,
        Board,
        Blank,
    }
    let mut state = State::Balls;

    let mut balls = vec![];
    let mut boards = vec![];
    let mut board_lines = vec![];

    for line in input.lines() {
        if line.len() == 0 {
            state = State::Blank;
        }
        match state {
            State::Balls => {
                balls = line.split(',').map(|s| s.parse::<u8>().unwrap()).collect();
            }
            State::Blank => {
                if board_lines.len() > 0 {
                    boards.push(Board::new(board_lines));
                }
                board_lines = vec![];
                state = State::Board;
            }
            State::Board => {
                board_lines.push(line.to_string());
            }
        }
    }

    println!("balls: {:?}", balls);
    println!("boards[0]: {:?}", boards[0]);
    println!("boards[final]: {:?}", boards[boards.len() - 1]);

    let mut ranked_boards: Vec<(usize, u64)> = boards.iter().map(|b| b.score(&balls)).collect();
    ranked_boards.sort();
    println!("Winner: {:?}", ranked_boards[0]);
    println!("Loser: {:?}", ranked_boards[ranked_boards.len() - 1]);
}

fn second(_input: &String) {
}

fn main() {
    let input = common::input::load_file("input.txt");
    first(&input);
    second(&input);
}
