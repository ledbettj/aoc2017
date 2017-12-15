mod day15 {
  pub struct Generator {
    previous: usize,
    factor: usize,
    modulo: usize
  }

  impl Generator {
    pub fn new(initial: usize, factor: usize) -> Generator {
      Generator { previous: initial, factor: factor, modulo: 2147483647 }
    }
  }

  impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
      let next = (self.previous * self.factor) % self.modulo;
      self.previous = next;
      Some(next)
    }

  }
}


use day15::*;

fn main() {
  let mut a = Generator::new(512, 16807);
  let mut b = Generator::new(191, 48271);

  let mut iter = a.zip(b);

  let n = 40_000_000;

  let valid = iter.take(40_000_000).filter(|&(aval, bval)|{
    (aval & 0xFFFF) == (bval & 0xFFFF)
  }).count();

  println!("{} valid generations", valid);
}
