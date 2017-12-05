use std::env;
use std::fs::File;
use std::io::*;
use std::collections::HashSet;

fn is_valid(phrase: &str) -> bool {
  let mut set = HashSet::new();

  phrase.split(" ").all(|word| set.insert(word))
}

fn is_valid_no_anagrams(phrase: &str) -> bool {
  let mut set = HashSet::new();

  phrase.split(" ").all(|word| {
    let mut chars : Vec<char> = word.chars().collect();
    chars.sort();

    let sorted : String = chars.iter().collect();
    set.insert(sorted)
  })
}


fn main() {
  let filename = env::args().skip(1).last().expect("You must provide a filename");

  let mut file = File::open(filename).expect("Failed to open file");
  let mut contents = String::new();

  file.read_to_string(&mut contents).expect("Failed to read file");

  let lines = contents.lines();

  let (valid, valid_anagrams) = lines.fold((0, 0), |(v, va), phrase| {
    match (is_valid(phrase), is_valid_no_anagrams(phrase)) {
      (true, true)  => (v + 1, va + 1),
      (true, false) => (v + 1, va),
      _ => (v, va)
    }
  });

  println!("part 1: {}; part 2: {}", valid, valid_anagrams)
}
