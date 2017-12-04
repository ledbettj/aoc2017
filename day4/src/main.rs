use std::env;
use std::fs::File;
use std::io::*;
use std::collections::HashSet;

fn valid_passphrase(phrase: &str) -> bool {
  let mut set = HashSet::new();

  for word in phrase.split(" ") {
    if set.contains(word) {
      return false;
    }

    set.insert(word);
  }

  true
}

fn valid_passphrase_no_angrams(phrase: &str) -> bool {
  let mut set = HashSet::new();

  for word in phrase.split(" ") {
    let mut chars : Vec<char> = word.chars().collect();
    chars.sort();

    let sorted : String = chars.iter().collect();
    if set.contains(&sorted) {
      return false;
    }

    set.insert(sorted);
  }

  true
}


fn main() {
  let args = env::args();
  let filename = args.skip(1).last().expect("You must provide a filename");
  let mut contents = String::new();

  match File::open(filename) {
    Ok(mut file) => {
      file.read_to_string(&mut contents);
      let lines = contents.lines();
      //let valid = lines.filter(|phrase| valid_passphrase(phrase)).count();
      let valid_angrams = lines.filter(|phrase| valid_passphrase_no_angrams(phrase)).count();

      //println!("{} passphrases are valid for part 1\n", valid);
      println!("{} passphrases are valid for part 2\n", valid_angrams);
    }

    Err(e) => panic!(e)
  }
  
}
