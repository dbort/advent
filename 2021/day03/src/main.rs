fn first(input: &String) {
    let mut zero_counts: Vec<u32> = vec![];
    let mut one_counts: Vec<u32> = vec![];
    for line in input.lines() {
        let num_bits = line.len();
        if zero_counts.len() == 0 {
            zero_counts = vec![0; num_bits];
            one_counts = vec![0; num_bits];
        }
        for (i, c) in line.chars().enumerate() {
            match c {
                '0' => zero_counts[i] += 1,
                '1' => one_counts[i] += 1,
                _ => panic!("Saw {}", c),
            }
        }
    }
    println!("zero_counts: {:?}", zero_counts);
    println!(" one_counts: {:?}", one_counts);

    let mut gamma = vec![];
    let mut epsilon = vec![];

    for (i, _) in zero_counts.iter().enumerate() {
        let zeros = zero_counts[i];
        let ones = one_counts[i];
        if zeros > ones {
            gamma.push('0');
            epsilon.push('1');
        } else if ones > zeros {
            gamma.push('1');
            epsilon.push('0');
        } else {
            panic!("Counts are the same for index {}", i);
        }
    }
    let gamma_binary: String = gamma.into_iter().collect();
    let gamma_int = i64::from_str_radix(&gamma_binary, 2).unwrap();
    let epsilon_binary: String = epsilon.into_iter().collect();
    let epsilon_int = i64::from_str_radix(&epsilon_binary, 2).unwrap();
    println!("  gamma: {:?} ({})", gamma_binary, gamma_int);
    println!("epsilon: {:?} ({})", epsilon_binary, epsilon_int);
    println!("power consumption: {}", gamma_int * epsilon_int);
}

fn second(input: &String) {
}

fn main() {
    let input = common::input::load_file("input.txt");
    first(&input);
    second(&input);
}
