pub fn string_to_digits(input: &str) -> Vec<u32> {
  input
    .chars()
    .map(|ch| ch.to_digit(10).unwrap())
    .collect()
}

pub fn select_matching_digits(list: &Vec<u32>) -> Vec<u32> {
  let mut dupe = list.clone();
  dupe.push(list[0]);

  dupe
    .windows(2)
    .filter(|pair| pair[0] == pair[1])
    .map(|pair| pair[0])
    .collect()
}

pub fn select_matching_halfway_round(list: &Vec<u32>) -> Vec<u32> {
  let offset = list.len() / 2;
  list.iter()
    .enumerate()
    .filter(|&(i, &value)| value == list[(i + offset) % list.len()])
    .map(|(_, &value)| value)
    .collect()
}

pub fn part1(input: &str) -> u32 {
  let values = string_to_digits(input);
  let digits  = select_matching_digits(&values);
  digits.iter().sum()
}

pub fn part2(input: &str) -> u32 {
  let values = string_to_digits(input);
  let digits  = select_matching_halfway_round(&values);
  digits.iter().sum()
}

