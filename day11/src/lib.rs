mod day11 {
  pub const INPUT : &'static str = include_str!("../input.txt");
  
  type Direction = (i32, i32, i32);
  const NORTH     : Direction = (0, 1, -1);
  const NORTHEAST : Direction = (1, 0, -1);
  const SOUTHEAST : Direction = (1, -1, 0);
  const SOUTH     : Direction = (0, -1, 1);
  const SOUTHWEST : Direction = (-1, 0, 1);
  const NORTHWEST : Direction = (-1, 1, 0);

  pub type Point = (i32, i32, i32);

  pub fn walk(start: &Point, path: &Vec<Direction>) -> Point {
    let mut p = start.clone();
    for dir in path {
      let (ref mut x, ref mut y, ref mut z) = p;
      let &(dx, dy, dz) = dir;
      *x += dx;
      *y += dy;
      *z += dz;
    }

    p
  }

  pub fn parse_path(path: &str) -> Vec<Direction> {
    path.split(",").map(|dir| {
      match dir.trim() {
        "n"  => NORTH,
        "ne" => NORTHEAST,
        "nw" => NORTHWEST,
        "s"  => SOUTH,
        "se" => SOUTHEAST,
        "sw" => SOUTHWEST,
        _    => panic!("unknown direction {}", dir)
      }
    }).collect()
  }

  pub fn hex_distance(a: &Point, b: &Point) -> u32 {
    let &(ax, ay, az) = a;
    let &(bx, by, bz) = b;

    ((ax - bx).abs() + (ay - by).abs() + (az - bz).abs()) as u32 / 2
  }
}

#[cfg(test)]
mod tests {
  use day11::*;

  #[test]
  fn test_examples() {
    let start : Point = (0, 0, 0);
    let mut path = parse_path("ne,ne,ne");
    let mut rc = walk(&start, &path);

    assert_eq!(hex_distance(&rc, &start), 3);

    path = parse_path("ne,ne,sw,sw");
    rc = walk(&start, &path);
    assert_eq!(hex_distance(&rc, &start), 0);

    path = parse_path("ne,ne,s,s");
    rc = walk(&start, &path);
    assert_eq!(hex_distance(&rc, &start), 2);

    path = parse_path("se,sw,se,sw,sw");
    rc = walk(&start, &path);
    assert_eq!(hex_distance(&rc, &start), 3);
  }

  #[test]
  fn part1() {
    let path = parse_path(INPUT);
    let start : Point = (0, 0, 0);

    let target = walk(&start, &path);

    assert_eq!(hex_distance(&target, &start), 805);
  }
}
