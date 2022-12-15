#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i64),
}

fn parse(input: &String) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        assert!(parts.len() > 0);
        let instr = match parts[0] {
            "noop" => Instruction::Noop,
            "addx" => Instruction::AddX(parts[1].parse::<i64>().unwrap()),
            _ => panic!("unknown instruction {}", parts[0]),
        };
        instructions.push(instr);
    }
    instructions
}

fn sample(cycle: usize, x: i64) -> i64 {
    if cycle >= 20 && (cycle - 20) % 40 == 0 {
        println!("Cycle {cycle}: X = {x}");
        cycle as i64 * x
    } else {
        0
    }
}

fn execute(instructions: &Vec<Instruction>) -> i64 {
    let mut strength: i64 = 0;
    let mut cycle: usize = 1;
    let mut x: i64 = 1;
    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                cycle += 1;
                strength += sample(cycle, x);
            }
            Instruction::AddX(val) => {
                cycle += 1;
                strength += sample(cycle, x);
                cycle += 1;
                x += val;
                strength += sample(cycle, x);
            }
        }
    }
    strength
}

fn first(input: &String) {
    let instructions = parse(input);
    let strength = execute(&instructions);
    println!("samples: {:?}", strength);
}

fn update(cycle: usize, x: i64) -> bool {
    let c = cycle as i64 % 40;
    c >= x - 1 && c <= x + 1
}

fn render(instructions: &Vec<Instruction>) -> Vec<bool> {
    let mut bits: Vec<bool> = Vec::new();
    let mut cycle: usize = 0;
    let mut x: i64 = 1;
    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                cycle += 1;
                bits.push(update(cycle, x));
            }
            Instruction::AddX(val) => {
                cycle += 1;
                bits.push(update(cycle, x));
                cycle += 1;
                x += val;
                bits.push(update(cycle, x));
            }
        }
    }
    bits
}

fn second(input: &String) {
    let instructions = parse(input);
    let bits = render(&instructions);
    for row in 0..6 {
        for col in 0..40 {
            print!(
                "{}",
                match bits[row * 40 + col] {
                    true => "#",
                    false => ".",
                }
            );
        }
        println!("");
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    first(&input);
    second(&input);
}
