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

    pub fn is_terminated(&self) -> bool {
      self.position < 0 || self.position as usize > self.instructions.len()
    }
  }
}

#[cfg(test)]
mod tests {
  use day5::JumpList;
  use std::env;

  #[test]
  fn it_works() {
    let filename = env::args().skip(1).last().expect("Provide input filename");
    let mut list = JumpList::new(&filename).expect("Failed to read jumplist");

    let mut n = 0;

    loop {
      list.step();
      n = n + 1;
      if list.is_terminated() {
        break;
      }
    }

    panic!("escaped after {} iterations", n);
  }
}
