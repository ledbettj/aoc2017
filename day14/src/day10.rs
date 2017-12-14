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

  pub fn full_hash(&mut self, instructions: &str) -> Vec<u32> {
    let mut values : Vec<usize> = instructions.chars().map(|ch| ch as usize).collect();
    for &i in [17, 31, 73, 47, 23].iter() {
      values.push(i);
    }
    for _ in 0..64 {
      self.hash(&values);
    }

    let dense : Vec<u32> = self.get().chunks(16).map(|chunk| {
      chunk.iter().fold(0 , |total, &value| total ^ value )
    }).collect();

    //    let parts : Vec<String> = dense.iter().map(|value| format!("{:02x}", value)).collect();
    //    parts.join("")
    dense
  }
}
