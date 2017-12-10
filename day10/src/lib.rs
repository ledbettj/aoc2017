mod day10 {
  pub struct CircularList {
    data: Vec<u32>,
    pub pos: usize,
    pub skip: usize
  }

  impl CircularList {

    pub fn get(&self) -> &Vec<u32> {
      &self.data
    }

    pub fn new(size: u32) -> CircularList {
      let items = (0..size).collect();

      CircularList { data: items, pos: 0, skip: 0 }
    }

    pub fn hash_step(&mut self, length: usize) {
      let list_length = self.data.len();
      let start = self.pos % list_length;
      let half  = length / 2;

      for i in 0..half {
        let index1 = (start + i) % list_length;
        let index2 = (start + length - i - 1) % list_length;
        self.data.swap(index1, index2);
      }

      self.pos += length + self.skip;
      self.skip += 1;
      self.pos %= list_length;
    }

    pub fn hash(&mut self, instructions: &Vec<usize>) {
      for &length in instructions {
        self.hash_step(length);
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use day10::*;

  #[test]
  fn part1() {
    let mut list = CircularList::new(256);
    list.hash(&vec![189,1,111,246,254,2,0,120,215,93,255,50,84,15,94,62]);

    let items : Vec<u32> = list.get().iter().take(2).cloned().collect();

    assert_eq!(items[0] * items[1], 38415);
  }

  #[test]
  fn it_works() {
    let mut list = CircularList::new(5);

    list.hash_step(3);
    assert_eq!(list.get(), &vec![2, 1, 0, 3, 4]);
    assert_eq!(list.pos, 3);
    assert_eq!(list.skip, 1);

    list.hash_step(4);
    assert_eq!(list.get(), &vec![4, 3, 0, 1, 2]);
    assert_eq!(list.pos, 3);
    assert_eq!(list.skip, 2);

    list.hash_step(1);
    assert_eq!(list.get(), &vec![4, 3, 0, 1, 2]);
    assert_eq!(list.pos, 1);
    assert_eq!(list.skip, 3);

    list.hash_step(5);
    assert_eq!(list.get(), &vec![3, 4, 2, 1, 0]);
    assert_eq!(list.pos, 4);
    assert_eq!(list.skip, 4);
  }
}
