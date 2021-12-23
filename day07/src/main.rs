fn cost(crabs: &Vec<usize>, position: usize) -> usize {
    let mut total = 0;
    for crab in crabs {
        total += (*crab as isize - position as isize).abs() as usize;
    }
    return total;
}

fn first(input: &String) {
    let mut crabs: Vec<usize> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    crabs.sort();
    let median = crabs[crabs.len() / 2];
    println!(
        "{} crabs, min {}, max {}, median {}",
        crabs.len(),
        crabs[0],
        crabs[crabs.len() - 1],
        median
    );
    let total = cost(&crabs, median);
    println!("Cost to median: {}", total);
}

fn cost2(crabs: &Vec<usize>, position: usize) -> usize {
    let mut total = 0;
    for crab in crabs {
        let steps = (*crab as isize - position as isize).abs() as usize;
        total += (steps * (steps + 1)) / 2;
    }
    return total;
}

fn second(input: &String) {
    let mut crabs: Vec<usize> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    crabs.sort();
    let min = crabs[0];
    let max = crabs[crabs.len() - 1];
    println!("{} crabs, min {}, max {}", crabs.len(), min, max);
    let mut costs = vec![];
    for posn in min..=max {
        costs.push((cost2(&crabs, posn), posn));
    }
    costs.sort();
    println!("Cost to {}: {}", costs[0].1, costs[0].0);
}

fn main() {
    let input = common::input::load_file("input.txt");
    first(&input);
    second(&input);
}
