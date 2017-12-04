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

fn main() {
  let args = env::args();
  let filename = args.skip(1).last().expect("You must provide a filename");
  let mut contents = String::new();

  match File::open(filename) {
    Ok(mut file) => {
      file.read_to_string(&mut contents);
      let lines = contents.lines();
      let valid = lines.filter(|phrase| valid_passphrase(phrase)).count();

      println!("{} passphrases are valid\n", valid);
    }

    Err(e) => panic!(e)
  }
  
}
