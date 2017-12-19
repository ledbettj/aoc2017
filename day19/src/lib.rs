mod day19 {
  pub type RoutingMap = Vec<Vec<char>>;

  pub fn load_routing_map(input: &str) -> RoutingMap {
    input
      .lines()
      .map(|line| line.chars().collect())
      .collect()
  }

  #[derive(Debug)]
  pub struct RoutingMapIterator {
    map: RoutingMap,
    pos: (isize, isize),
    dir: (isize, isize)
  }

  impl RoutingMapIterator {

    pub fn new(map: RoutingMap) -> RoutingMapIterator {
      let mut iter = RoutingMapIterator{
        map: map,
        pos: (0, 0),
        dir: (1, 0)
      };
      iter.locate_start();
      iter
    }

    fn locate_start(&mut self) {
      let (col, _) = self.map[0].iter()
        .enumerate()
        .find(|&(_, &value)| value == '|')
        .expect("Failed to find start");

      self.pos = (0, col as isize);
    }

    fn is_valid_pos(map: &RoutingMap, r: isize, c: isize) -> bool {
      /* moved outside the map */
      if r < 0 || r as usize >= map.len() || c < 0 || c as usize >= map[0].len() {
        return false;
      }

      /* on a space */
      let ch = map[r as usize][c as usize];
      !ch.is_whitespace()
    }
  }

  impl Iterator for RoutingMapIterator {
    type Item = char;

    fn next(&mut self) -> Option<char> {
      let (curr, curc) = self.pos;

      if !RoutingMapIterator::is_valid_pos(&self.map, curr, curc) {
        return None;
      }

      let (ref mut r, ref mut c) = self.pos;
      let (ref mut dr, ref mut dc) = self.dir;


      let ch = self.map[*r as usize][*c as usize];

      match ch {
        '+' => {
          /* we can either turn left or right */
          let (r1, c1) = (*r + *dc, *c + *dr);
          let (r2, c2) = (*r - *dc, *c - *dr);
          let tmp : isize = *dc;

          if RoutingMapIterator::is_valid_pos(&self.map, r1, c1) {
            *r = r1;
            *c = c1;
            *dc = *dr;
            *dr = tmp;
          } else if RoutingMapIterator::is_valid_pos(&self.map, r2, c2) {
            *r = r2;
            *c = c2;
            *dc = -*dr;
            *dr = -tmp;
          } else {
            panic!("Nowhere to turn to");
          }
        },
        _   => {
          /* anything else - keep going same direction  */
          *r += *dr;
          *c += *dc
        }
      };

      Some(ch)
    }

  }
}

#[cfg(test)]
mod tests {
  const INPUT : &str = include_str!("../input.txt");
  const TEST : &str  = include_str!("../test.txt");

  use day19::*;

  #[test]
  fn test_example() {
    let map = load_routing_map(TEST);
    let iter = RoutingMapIterator::new(map);

    let result = iter
      .filter(|value| value.is_alphanumeric())
      .collect::<String>();

    assert_eq!(result, "ABCDEF");
  }


  #[test]
  fn test_example_2() {
    let map = load_routing_map(TEST);
    let iter = RoutingMapIterator::new(map);

    let result = iter.count();

    assert_eq!(result, 38);
  }

  #[test]
  fn test_p1() {
    let map = load_routing_map(INPUT);
    let iter = RoutingMapIterator::new(map);

    let result = iter
      .filter(|value| value.is_alphanumeric())
      .collect::<String>();

    assert_eq!(result, "EPYDUXANIT");
  }

  #[test]
  fn test_p2() {
    let map = load_routing_map(INPUT);
    let iter = RoutingMapIterator::new(map);

    let result = iter.count();

    assert_eq!(result, 17544);
  }
}
