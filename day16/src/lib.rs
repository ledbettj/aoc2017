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
          // this is incredibly inefficient
          rest.reverse();
          for ch in rest { p.insert(0, ch) }
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

  const INPUT : &str = include_str!("../input.txt");
  #[test]
  fn it_works() {
    let mut p = programs_new();
    let dance = dance_new(INPUT);

    do_dance(&mut p, &dance);

    let s = p.iter().collect::<String>();
    assert_eq!(s, "kpfonjglcibaedhm");
  }
}
