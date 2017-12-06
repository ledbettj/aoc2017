mod day6 {
  use std::collections::HashSet;
  use std::num;

  #[derive(Debug, Clone, Hash, PartialEq, Eq)]
  pub struct MemoryBanks {
    banks: Vec<u32>
  }

  impl MemoryBanks {
    pub fn new(input: &str) -> Result<MemoryBanks, num::ParseIntError> {
      let banks = input
        .split_whitespace()
        .map(|token| token.parse::<u32>() )
        .collect();

      match banks {
        Ok(b)  => Ok(MemoryBanks{banks: b}),
        Err(e) => Err(e)
      }
    }

    fn find_balance_target(&self) -> (usize, u32) {
      let (pos, &value) = self.banks.iter()
        .enumerate()
        .max_by_key(|&(i, v)| (v, -(i as i32)))
        .expect("Empty bank provided");

      (pos, value)
    }

    pub fn rebalance(&mut self) {
      let (index, value) = self.find_balance_target();
      let len = self.banks.len();

      self.banks[index] = 0;

      let mut count = value;
      let mut pos = (index + 1) % len;

      while count > 0 {
        self.banks[pos] += 1;
        pos = (pos + 1) % len;
        count -= 1;
      }
    }
  }


  pub fn count_to_cycle_banks(mut banks: &mut MemoryBanks) -> usize {
    let mut set = HashSet::new();
    let mut count = 0;
    set.insert(banks.clone());

    loop {
      banks.rebalance();
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
  fn test_rebalance() {
    let mut b = MemoryBanks::new("0 3 1 3").unwrap();
    let t = MemoryBanks::new("1 0 2 4").unwrap();

    b.rebalance();
    assert_eq!(b, t);
  }
  
  #[test]
  fn part1() {
    let mut b = MemoryBanks::new("10	3	15	10	5	15	5	15	9	2	5	8	5	2	3	6").unwrap();

    assert_eq!(count_to_cycle_banks(&mut b), 14029);
  }

  #[test]
  fn part2() {
    let mut b = MemoryBanks::new("10	3	15	10	5	15	5	15	9	2	5	8	5	2	3	6").unwrap();
    count_to_cycle_banks(&mut b);

    let target = b.clone();
    let mut count = 0;
    loop {
      b.rebalance();
      count += 1;

      if b == target {
        break;
      }
    }

    assert_eq!(count, 2765);
  }
}
