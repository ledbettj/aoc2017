mod day3 {
  pub fn manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    (x2 - x1).abs() + (y2 - y1).abs()
  }

  // https://stackoverflow.com/questions/10094745
  pub fn grid_location(n: i32) -> (i32, i32) {
    let k = (((n as f32).sqrt() - 1.0) / 2.0).ceil() as i32;
    let mut t = 2 * k + 1;
    let mut m = t * t;

    t -= 1;

    if n >= m - t {
      return (k - (m - n), -k);
    }

    m -= t;

    if n >= m - t {
      return (-k, -k + (m - n));
    }

    m -= t;

    if n >= m - t {
      return (-k + (m - n), k);
    }

    (k, k - ( m - n - t))
  }
}

#[cfg(test)]
mod tests {
  use day3;

  #[test]
  fn part1() {
    let pos = day3::grid_location(325489);

    assert_eq!(day3::manhattan_distance(pos, (0, 0)), 552);
  }

  #[test]
  fn test_manhattan_distance() {
    assert_eq!(day3::manhattan_distance((1, 1), (0, 0)), 2);
    assert_eq!(day3::manhattan_distance((2, 1), (0, 0)), 3);
  }

  #[test]
  fn test_grid_location() {
    /*
    37  36  35  34  33  32 31
    38  17  16  15  14  13 30
    39  18   5   4   3  12 29
    40  19   6   1   2  11 28
    41  20   7   8   9  10 27
    42  21  22  23  24  25 26
    43  44  45  46  47  48 49
     */
    assert_eq!(day3::grid_location(1), (0, 0));
    assert_eq!(day3::grid_location(2), (1, 0));
    assert_eq!(day3::grid_location(3), (1, 1));
    assert_eq!(day3::grid_location(4), (0, 1));
    assert_eq!(day3::grid_location(5), (-1, 1));
    assert_eq!(day3::grid_location(30), (3, 2));
    assert_eq!(day3::grid_location(37), (-3, 3));
    assert_eq!(day3::grid_location(49), (3, -3));
  }
}


