mod day16 {
  pub enum DanceMoves {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char)
  }

  pub type Dance = Vec<DanceMoves>;

  pub type Programs = Vec<char>;

  pub fn programs_new() -> Programs {
    "abcdefghijklmnop".chars().collect()
  }

  fn parse_dance_move(m: &str) -> DanceMoves {
      let mut chars = m.chars();
      let ch : char = chars.next().unwrap();

      match ch {
        's' => DanceMoves::Spin(m[1..].parse::<usize>().unwrap()),
        'x' => {
          let params : Vec<usize> = m[1..]
            .split('/')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
          DanceMoves::Exchange(params[0], params[1])
        }
        'p' => {
          let params : Vec<char> = chars.take(3).collect();
          DanceMoves::Partner(params[0], params[2])
        }
        v   => panic!("invalid value: {}", v)
      }
  }

  pub fn dance_new(moves: &str) -> Dance {
    moves
      .trim()
      .split(",")
      .map(|m| parse_dance_move(m))
      .collect()
  }

  pub fn do_dance(p: &mut Programs, dance: &Dance) {
    for m in dance {
      match m {
        &DanceMoves::Spin(v) => {
          let offset = p.len() - v;
          let mut rest = p.split_off(offset);
          rest.append(p);
          *p = rest;
        },
        &DanceMoves::Exchange(a, b) => {
          p.swap(a, b)
        }
        &DanceMoves::Partner(a, b) => {
          let positions : Vec<usize> = p.iter()
            .enumerate()
            .filter(|&(_, &ch)| ch == a || ch == b)
            .map(|(index, _)| index)
            .collect();
          p.swap(positions[0], positions[1]);
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use day16::*;
  use std::collections::HashMap;

  const INPUT : &str = include_str!("../input.txt");
  #[test]
  fn it_works() {
    let mut p = programs_new();
    let dance = dance_new(INPUT);

    do_dance(&mut p, &dance);

    let s = p.iter().collect::<String>();
    assert_eq!(s, "kpfonjglcibaedhm");
  }

  #[test]
  fn p2_test() {
    let mut p = programs_new();
    let dance = dance_new(INPUT);
    let mut modu = 0;
    let mut set: HashMap<String, usize> = HashMap::new();

    for i in 0..1_000_000_000 {
      let s = p.iter().collect::<String>();

      if set.contains_key(&s) {
        let k = set[&s];
        //println!("value at {} is same as value at {}", i, k);
        //assert_eq!(false, true);
        modu = (i - k);
        break;
      }

      set.insert(s, i);
      do_dance(&mut p, &dance);
    }

    let count = 1_000_000_000 % modu;
    for _ in 0..count {
      do_dance(&mut p, &dance);
    }
    let s = p.iter().collect::<String>();
    assert_eq!(s, "odiabmplhfgjcekn");
  }
}
