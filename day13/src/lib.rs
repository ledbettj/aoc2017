mod day13 {
  use std::num;

  pub struct ScannerArray {
    pub scanners: Vec<usize>
  }

  impl ScannerArray {
    pub fn parse(input: &str) -> Result<ScannerArray, num::ParseIntError> {
      let mut scanners : Vec<usize> = Vec::new();
      for line in input.lines() {
        let parts : Vec<usize> = line
          .trim()
          .split(": ")
          .map(|value| value.parse::<usize>())
          .collect::<Result<Vec<usize>, _>>()?;

        if !scanners.len() > parts[0] {
          scanners.resize(parts[0] + 1, 0)
        }

        scanners[parts[0]] = parts[1];
      }
      Ok(ScannerArray { scanners: scanners })
    }

    pub fn is_safe_traversal(&self, offset : usize) -> bool {
      for (index, &range) in self.scanners.iter().enumerate() {
        if range == 0 {
          continue;
        }

        if (index + offset) % ((range -1) *2) == 0 {
          return false;
        }
      }

      true
    }

    pub fn traverse_severity(&self) -> usize {
      let mut score : usize = 0;

      for (index, &range) in self.scanners.iter().enumerate() {
        if range == 0 {
          continue;
        }

        if index % ((range -1) *2) == 0 {
          score += index * range;
        }
      }

      score
    }
  }
}

#[cfg(test)]
mod tests {
  use day13::*;

  const INPUT : &str = include_str!("../input.txt");

  #[test]
  fn test_input() {
    let input = "0: 3
1: 2
4: 4
6: 4";

    let s = ScannerArray::parse(input).expect("Failed to parse input");
    assert_eq!(s.traverse_severity(), 24);

    for i in 0..10 {
      assert_eq!(s.is_safe_traversal(i), false);
    }

    assert_eq!(s.is_safe_traversal(10), true);
  }

  #[test]
  fn test_p1() {
    let s = ScannerArray::parse(INPUT).expect("Failed to parse input");
    assert_eq!(s.traverse_severity(), 1580);
  }

  #[test]
  fn test_p2() {
    let s = ScannerArray::parse(INPUT).expect("Failed to parse input");
    let mut t = 0;

    loop {
      if s.is_safe_traversal(t) { break; }
      t += 1;
    }

    assert_eq!(t, 0);
  }
}
