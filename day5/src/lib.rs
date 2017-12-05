mod day5 {
  use std::fs::File;
  use std::io::*;

  pub struct JumpList {
    instructions: Vec<i32>,
    position: i32
  }

  impl JumpList {
    pub fn new(filename: &str) -> Result<JumpList> {

      let mut contents = String::new();
      let mut file = File::open(filename)?;

      file.read_to_string(&mut contents)?;

      let list = contents
        .lines()
        .map(|line| line.parse::<i32>().expect("Invalid number"))
        .collect();

      Ok(JumpList{instructions: list, position: 0})
    }

    pub fn step(&mut self) -> &Self {
      let old_position = self.position as usize;
      let offset = self.instructions[self.position as usize];
      self.position += offset;
      self.instructions[old_position] += 1;

      self
    }

    pub fn step_odd(&mut self) -> &Self {
      let old_position = self.position as usize;
      let offset       = self.instructions[old_position];

      self.position += offset;

      if offset >= 3 {
        self.instructions[old_position] -= 1;
      } else {
        self.instructions[old_position] += 1;
      }
      self
    }

    pub fn is_terminated(&self) -> bool {
      self.position < 0 || self.position as usize >= self.instructions.len()
    }
  }
}

#[cfg(test)]
mod tests {
  use day5::JumpList;
  const FILENAME : &str = "./input.txt";

  #[test]
  pub fn test_part1() {
    let mut list = JumpList::new(FILENAME).expect("Failed to read jumplist");

    let mut n = 0;

    loop {
      list.step();
      n = n + 1;
      if list.is_terminated() {
        break;
      }
    }

    assert_eq!(n, 358131);
  }

  #[test]
  pub fn test_part2() {
    let mut list = JumpList::new(FILENAME).expect("Failed to read jumplist");

    let mut n = 0;

    loop {
      list.step_odd();
      n = n + 1;
      if list.is_terminated() {
        break;
      }
    }

    assert_eq!(n, 25558839);
  }
}
