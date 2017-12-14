mod day10;

mod day14 {
  use day10::*;
  use std::collections::HashSet;

  pub type Grid = [[u8; 128]; 128];

  pub fn build_grid(initializer: &str) -> Grid {
    let mut grid : Grid = [[0u8; 128]; 128];

    for i in 0..128 {
      let key = format!("{}-{}", initializer, i);
      let mut h = CircularList::new(256);
      let result = h.full_hash(&key);

      for (index, value) in result.iter().enumerate() {
        for bit in 0..8 {
          let pos = (index * 8) + bit;
          let v = value >> (7 - bit) & 1;
          grid[i][pos] = v as u8;
        }
      }
    }

    grid
  }

  pub fn print_grid(g: &Grid, w: usize, h: usize) {
    for i in 0..h {
      for j in 0..w {
        let ch = match g[i][j] {
          0 => '.',
          _ => '#'
        };
        print!("{}", ch);
      }
      println!("");
    }
  }

  pub fn count_regions(g: &Grid, w: usize, h: usize) -> usize {
    let mut seen : HashSet<(usize, usize)> = HashSet::new();
    let mut count = 0;
    let mut neighbors : Vec<(usize, usize)> = Vec::new();

    for i in 0..h {
      for j in 0..h {
        let value = g[i][j];
        /* empty or already visited this cell */
        if value == 0 || !seen.insert((i, j)) {
          continue;
        }

        /* new group */
        count += 1;
        /* mark all cells in this group */
        for a in -1..2 {
          for b in -1..2 {
            /* ignore diagonals and self */
            if (a != 0 && b != 0) || (a == 0 && b == 0) {
              continue;
            }
            let r = i as isize + a;
            let c = j as isize + b;
            if r >= 0 && r < h as isize && c >= 0 && c < w as isize {
              neighbors.push((r as usize, c as usize));
            }
        }

      }
    }

    count
  }
}

#[cfg(test)]
mod tests {
  const INITIALIZER : &str = "vbqugkhl";

  use day14::*;
  use day10::*;

  #[test]
  fn test_example() {
    let g = build_grid("flqrgnkx");
    print_grid(&g, 8, 8);
  }

  #[test]
  fn it_works() {
    let g = build_grid(INITIALIZER);
    let count = g.iter()
      .map(|row| row.iter().sum::<u8>() as u32)
      .sum::<u32>();

    assert_eq!(count, 8148);
  }
}
