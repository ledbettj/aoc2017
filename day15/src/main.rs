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

  pub struct StrictGenerator {
    previous: usize,
    factor: usize,
    strict: usize,
    modulo: usize
  }

  impl StrictGenerator {
    pub fn new(initial: usize, factor: usize, strict: usize) -> StrictGenerator {
      StrictGenerator { previous: initial, factor: factor, strict: strict, modulo: 2147483647 }
    }
  }

  impl Iterator for StrictGenerator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
      let mut next = (self.previous * self.factor) % self.modulo;

      while next % self.strict != 0 {
        self.previous = next;
        next = (self.previous * self.factor) % self.modulo;
      }

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

  let valid = iter.take(40_000_000).filter(|&(aval, bval)|{
    (aval & 0xFFFF) == (bval & 0xFFFF)
  }).count();

  println!("{} valid generations", valid);

  let mut a2 = StrictGenerator::new(512, 16807, 4);
  let mut b2 = StrictGenerator::new(191, 48271, 8);
  let mut iter2 = a2.zip(b2);

  let valid2 = iter2.take(5_000_000).filter(|&(aval, bval)|{
    (aval & 0xFFFF) == (bval & 0xFFFF)
  }).count();

  println!("{} valid generations part 2", valid2);

}


#[test]
fn test_part2() {
  let mut a = StrictGenerator::new(65, 16807, 4);
  let mut b = StrictGenerator::new(8921, 48271, 8);

  assert_eq!(a.next(), Some(1352636452));
  assert_eq!(b.next(), Some(1233683848));

  assert_eq!(a.next(), Some(1992081072));
  assert_eq!(b.next(), Some(862516352));

}
