fn get_sums(input: &String) -> Vec<i64> {
  let mut ret = Vec::new();
  let mut cal_sum: i64 = 0;
  for line in input.lines() {
    if line.is_empty() {
      ret.push(cal_sum);
      cal_sum = 0;
    } else {
      cal_sum += line.parse::<i64>().unwrap();
    }
  }
  ret.push(cal_sum);
  return ret;
}

fn first(input: &String) {
    let sums = get_sums(input);
    assert!(sums.len() >= 1);
    println!("Max calories: {}", sums.iter().max().unwrap());
}

fn second(input: &String) {
    let mut sums = get_sums(input);
    sums.sort_unstable_by(|a, b| b.cmp(a)); // Reverse
    assert!(sums.len() >= 3);
    println!("Max calories: {} + {} + {} = {}", sums[0], sums[1], sums[2], 
    sums[0] +  sums[1] + sums[2]);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    first(&input);
    second(&input);
}
