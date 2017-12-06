mod day6 {
  use std::collections::HashSet;

  pub fn initialize_banks(input: &str) -> Vec<u32> {
    input
      .split_whitespace()
      .map(|token| token.parse::<u32>().expect("Invalid value"))
      .collect()
  }

  pub fn target_bank(banks: &Vec<u32>) -> (usize, u32) {
    let (pos, &value) = banks.iter()
      .enumerate()
      .max_by_key(|&(i, v)| (v, -(i as i32)))
      .expect("Empty bank provided");

    (pos, value)
  }

  pub fn rebalance_banks(banks: &mut Vec<u32>) {
    let (index, value) = target_bank(&banks);
    let len = banks.len();

    banks[index] = 0;

    let mut count = value;
    let mut pos = (index + 1) % len;

    while count > 0 {
      banks[pos] += 1;
      pos = (pos + 1) % len;
      count -= 1;
    }
  }

  pub fn count_to_cycle_banks(mut banks: &mut Vec<u32>) -> usize {
    let mut set = HashSet::new();
    let mut count = 0;
    set.insert(banks.clone());

    loop {
      rebalance_banks(&mut banks);
      count += 1;

      if !set.insert(banks.clone()) {
        break;
      }
    }

    count
  }

}
#[cfg(test)]
mod tests {
  use day6::*;
  #[test]
  fn it_works() {
    assert_eq!(initialize_banks("1 2\t3"), [1, 2, 3]);

    let mut b = initialize_banks("1 2 1 2");
    rebalance_banks(&mut b);
    assert_eq!(b, [1, 0, 2, 3]);
    rebalance_banks(&mut b);
    assert_eq!(b, [2, 1, 3, 0]);

    let mut b2 = initialize_banks("0 2 7 0");
    assert_eq!(count_to_cycle_banks(&mut b2), 5);
  }

  #[test]
  fn part1() {
    let mut b = initialize_banks("10	3	15	10	5	15	5	15	9	2	5	8	5	2	3	6");

    assert_eq!(count_to_cycle_banks(&mut b), 14029);
  }
}
