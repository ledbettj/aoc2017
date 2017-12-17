mod day17 {
  use std::collections::VecDeque;
  use std::collections::vec_deque;

  pub struct Cyclone {
    list: VecDeque<u32>,
    position: usize,
    step:     usize
  }

  impl Cyclone {
    pub fn new(step: usize) -> Cyclone {
      let mut list = VecDeque::with_capacity(50);
      list.push_front(0);

      Cyclone{
        list:     list,
        position: 0,
        step:     step
      }
    }

    pub fn step(&mut self, value: u32) {
      let index = (self.position + self.step) % self.list.len() + 1;
      self.list.insert(index, value);
      self.position = index;
    }

    pub fn iter(&self) -> vec_deque::Iter<u32> {
      self.list.iter()
    }

    pub fn as_vec(&self) -> Vec<u32> {
      self.iter().cloned().collect::<Vec<u32>>()
    }

    pub fn get(&self, index: usize) -> u32 {
      let i = index % self.list.len();
      self.list[i]
    }

  }
}

#[cfg(test)]
mod tests {
  use day17::*;

  #[test]
  fn p1_example() {
    let mut c = Cyclone::new(3);

    c.step(1);
    assert_eq!(c.as_vec(), [0, 1]);
    c.step(2);
    assert_eq!(c.as_vec(), [0, 2, 1]);
    c.step(3);
    assert_eq!(c.as_vec(), [0, 2, 3, 1]);
    c.step(4);
    assert_eq!(c.as_vec(), [0, 2, 4, 3, 1]);
    c.step(5);
    assert_eq!(c.as_vec(), [0, 5, 2, 4, 3, 1]);
    c.step(6);
    assert_eq!(c.as_vec(), [0, 5, 2, 4, 3, 6, 1]);
    c.step(7);
    assert_eq!(c.as_vec(), [0, 5, 7, 2, 4, 3, 6, 1]);
    c.step(8);
    assert_eq!(c.as_vec(), [0, 5, 7, 2, 4, 3, 8, 6, 1]);
    c.step(9);
    assert_eq!(c.as_vec(), [0, 9, 5, 7, 2, 4, 3, 8, 6, 1]);
  }

  #[test]
  fn p1_example_2() {
    let mut c = Cyclone::new(3);


    for i in 1..2018 {
      c.step(i);
    }

    let (pos, _) = c.iter().enumerate().find(|&(_, &value)| value == 2017).unwrap();

    assert_eq!(c.get(pos + 1), 638);
  }


  #[test]
  fn part1() {
    let mut c = Cyclone::new(328);

    for i in 1..2018 {
      c.step(i);
    }

    let (pos, _) = c.iter().enumerate().find(|&(_, &value)| value == 2017).unwrap();

    assert_eq!(c.get(pos + 1), 1670);

  }

  #[test]
  fn part2() {
    let mut len = 1;
    let step    = 328;
    let mut pos = 0;

    let v : u32 = (1..50_000_001).filter(|i| {
      let index = (pos + step) % len + 1;
      pos = index;
      len += 1;

      index == 1
    }).last().unwrap();

    assert_eq!(v, 2316253);

  }
}
