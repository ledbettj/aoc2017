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

  pub fn grid_neighbors(grid: &Grid, pos: (usize, usize), w: usize, h: usize) -> Vec<(usize, usize)> {
    let (_r, _c)  = pos;
    let or = _r as isize;
    let oc = _c as isize;

    let mut rc = Vec::new();
    for a in -1..2 {
      for b in -1..2 {
        if (a == 0 && b == 0) || (a != 0 && b != 0) {
          continue;
        }
        let r = or + a;
        let c = oc + b;
        if r < 0 || r >= h as isize || c < 0 || c >= w as isize {
          continue;
        }

        if grid[r as usize][c as usize] != 0 {
          rc.push((r as usize, c as usize));
        }
      }
    }
    rc
  }

  pub fn count_regions(g: &Grid, w: usize, h: usize) -> usize {
    let mut count = 0;

    let mut to_visit : Vec<(usize, usize)> = Vec::new();
    let mut seen : HashSet<(usize, usize)> = HashSet::new();

    for i in 0..h {
      for j in 0..w {
        if g[i][j] != 0 {
          to_visit.push((i, j));
        }
      }
    }

    while !to_visit.is_empty() {
      let (r, c) = to_visit.pop().unwrap();
      if !seen.insert((r, c)) {
        continue;
      }

      count += 1;

      let mut neighbors : Vec<(usize, usize)> = grid_neighbors(&g, (r, c), w, h);

      while !neighbors.is_empty() {
        let (nr, nc) = neighbors.pop().unwrap();
        if seen.insert((nr, nc)) {
          let mut x = grid_neighbors(&g, (nr, nc), w, h);
          neighbors.append(&mut x);
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

  #[test]
  fn it_works_p2() {
    let g = build_grid(INITIALIZER);
    let count = count_regions(&g, 128, 128);

    assert_eq!(count, 0);
  }
}
