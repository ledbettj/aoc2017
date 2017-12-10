use std::fs::File;
use std::io::*;
use std::env;

enum ParseState {
  Normal,
  Ignore,
  Garbage
}

#[test]
fn test_day9() {
  assert_eq!(day9("{}"), (1, 0));
  assert_eq!(day9("{{<a!>},{<a!>},{<a!>},{<ab>}}"), (3, 17));
}

fn main() {
  let filename = env::args().skip(1).last().expect("Provide input filename");
  let mut file = File::open(filename).expect("Failed to open file");
  let mut contents = String::new();

  file.read_to_string(&mut contents).expect("Failed to read file");

  let (score, garbage) = day9(&contents);
  println!("score: {} garbage: {}", score, garbage);
}

fn day9(input: &str) -> (usize, usize) {
  let mut state = ParseState::Normal;
  let mut score : usize = 0;
  let mut value : usize = 1;
  let mut garbage : usize = 0;

  for ch in input.chars() {
    state = match state {
      ParseState::Normal => {
        match ch {
          '{'  => { score += value; value += 1; ParseState::Normal },
          '}'  => { value -= 1; ParseState::Normal },
          '<'  => ParseState::Garbage,
          ','  => ParseState::Normal,
          '\n' => ParseState::Normal,
          _    => panic!("Unexpected character: '{}'", ch)
        }
      },
      ParseState::Garbage => {
        match ch {
          '!' => ParseState::Ignore,
          '>' => ParseState::Normal,
          _   => { garbage += 1; ParseState::Garbage }
        }
      },
      ParseState::Ignore => ParseState::Garbage
    };
  }

  (score, garbage)
}
