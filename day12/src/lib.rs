mod day12 {
  use std::collections::HashMap;
  use std::collections::HashSet;

  pub const INPUT : &'static str = include_str!("../input.txt");

  fn handle_line(data : &mut HashMap<u32, HashSet<u32>>, line: &str) {
    let parts : Vec<&str> = line.split(" <-> ").collect();
    let lhs = parts[0].parse::<u32>().expect("Bad value");
    let rhs : Vec<u32> = parts[1].trim()
      .split(", ")
      .map(|v| v.parse::<u32>().expect("Bad value"))
      .collect();

    let set = HashSet::new();

    for value in &rhs {
      let entry = data.entry(*value).or_insert(set.clone());
      entry.insert(lhs);
    }

    let lh_entry = data.entry(lhs).or_insert(set.clone());

    for value in rhs {
      lh_entry.insert(value);
    }
  }

  pub fn parse_graph(input: &str) -> HashMap<u32, HashSet<u32>> {
    let mut results = HashMap::<u32, HashSet<u32>>::new();

    for line in input.lines() {
      handle_line(&mut results, line);
    }

    results
  }

  pub fn count_groups(graph: &mut HashMap<u32, HashSet<u32>>) -> u32 {
    let mut count = 0;

    while !graph.is_empty() {
      let (&start, _) = graph.iter().last().unwrap();
      remove_group(graph, start);
      count += 1;
    }

    count
  }

  pub fn remove_group(graph: &mut HashMap<u32, HashSet<u32>>, start: u32) {
    let mut seen : HashSet<u32> = HashSet::new();
    let mut stack : Vec<u32> = vec![start];

    while !stack.is_empty() {
      let v = stack.pop().unwrap();
      {
        if let Some(neighbors) = graph.get(&v) {

          for n in neighbors {
            if seen.insert(*n) {
              stack.push(*n);
            }
          }
        }
      }
      graph.remove(&v);
    }

  }

  pub fn count_neighbors(graph: &HashMap<u32, HashSet<u32>>, start: u32) -> u32 {
    let mut seen : HashSet<u32> = HashSet::new();
    let mut stack = vec![start];
    let mut count = 0;

    while !stack.is_empty() {
      let v = stack.pop().unwrap();
      let neighbors = graph.get(&v).unwrap();

      for n in neighbors {
        if seen.insert(*n) {
          count += 1;
          stack.push(*n);
        }
      }
    }

    count
  }
}

#[cfg(test)]
mod tests {
  use day12::*;

  #[test]
  fn test_sample() {
    let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

    let mut data = parse_graph(input);

    assert_eq!(data.get(&2).unwrap().len(), 3);

    assert_eq!(count_neighbors(&data, 0), 6);
    assert_eq!(count_neighbors(&data, 1), 1);

    assert_eq!(count_groups(&mut data), 2);
  }

  #[test]
  fn part1() {
    let mut data = parse_graph(INPUT);
    assert_eq!(count_neighbors(&data, 0), 113);

    assert_eq!(count_groups(&mut data), 202);
  }
}
