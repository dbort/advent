#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn new(spec: &Vec<&str>) -> Self {
        let ids: Vec<usize> = spec
            .last()
            .unwrap()
            .split(' ')
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let num_stacks: usize = *ids.last().unwrap();
        println!("{num_stacks} stacks");
        let mut ret = Stacks {
            stacks: std::iter::repeat(Vec::new()).take(num_stacks).collect(),
        };
        for i in (0..(spec.len() - 1)).rev() {
            for j in 0..num_stacks {
                let c = spec[i].as_bytes()[j * 4 + 1] as char;
                if c != ' ' {
                    ret.stacks[j].push(c);
                }
            }
        }
        ret
    }

    fn execute(&mut self, command: &str) {
        let nums: Vec<usize> = command
            .split(' ')
            .filter_map(|s| match s.parse::<usize>() {
                Ok(n) => Some(n),
                Err(_) => None,
            })
            .collect();
        if nums.len() != 3 {
            panic!("Bad command {command}");
        }
        let count = nums[0];
        let src = nums[1] - 1;
        let dst = nums[2] - 1;
        for _ in 0..count {
            let c = self.stacks[src].pop().unwrap();
            self.stacks[dst].push(c);
        }
    }

    fn execute2(&mut self, command: &str) {
        let nums: Vec<usize> = command
            .split(' ')
            .filter_map(|s| match s.parse::<usize>() {
                Ok(n) => Some(n),
                Err(_) => None,
            })
            .collect();
        if nums.len() != 3 {
            panic!("Bad command {command}");
        }
        let count = nums[0];
        let src = nums[1] - 1;
        let dst = nums[2] - 1;
        let mut v: Vec<char> = Vec::new();
        for _ in 0..count {
            v.push(self.stacks[src].pop().unwrap());
        }
        for c in v.iter().rev() {
            self.stacks[dst].push(*c);
        }
    }
}

fn first(input: &String) {
    let mut stack_spec: Vec<&str> = Vec::new();
    let mut getting_stacks: bool = true;
    let mut stacks = Stacks { stacks: Vec::new() };
    for line in input.lines() {
        if getting_stacks {
            if line.len() > 0 {
                stack_spec.push(line);
            } else {
                stacks = Stacks::new(&stack_spec);
                println!("Made stacks {:?}", stacks);
                getting_stacks = false;
            }
        } else {
            stacks.execute(line);
        }
    }
    println!("Final stacks {:?}", stacks);
    let mut tops = String::new();
    for stack in stacks.stacks {
        tops.push(*stack.last().unwrap());
    }
    println!("Tops: {tops}");
}

fn second(input: &String) {
    let mut stack_spec: Vec<&str> = Vec::new();
    let mut getting_stacks: bool = true;
    let mut stacks = Stacks { stacks: Vec::new() };
    for line in input.lines() {
        if getting_stacks {
            if line.len() > 0 {
                stack_spec.push(line);
            } else {
                stacks = Stacks::new(&stack_spec);
                println!("Made stacks {:?}", stacks);
                getting_stacks = false;
            }
        } else {
            stacks.execute2(line);
        }
    }
    println!("Final stacks {:?}", stacks);
    let mut tops = String::new();
    for stack in stacks.stacks {
        tops.push(*stack.last().unwrap());
    }
    println!("Tops: {tops}");
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    first(&input);
    second(&input);
}
