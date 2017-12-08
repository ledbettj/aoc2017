mod day8 {
  use std::collections::HashMap;
  use std::fs::File;
  use std::io::*;

  fn eq(a: i32, b: i32) -> bool {
    a == b
  }

  fn neq(a: i32, b: i32) -> bool {
    a != b
  }

  fn gt(a: i32, b: i32) -> bool {
    a > b
  }

  fn gte(a: i32, b: i32) -> bool {
    a >= b
  }

  fn lt(a: i32, b: i32) -> bool {
    a < b
  }

  fn lte(a: i32, b: i32) -> bool {
    a <= b
  }


  type Comparator = fn(i32, i32) -> bool;

  enum Operator {
    IncOperator,
    DecOperator
  }

  pub struct Condition {
    register: String,
    pub comp: Comparator,
    value: i32
  }

  pub struct Instruction {
    register:  String,
    operator:  Operator,
    value:     i32,
    condition: Condition
  }

  pub struct Computer {
    pub registers: HashMap<String, i32>,
    pub max_value: i32
  }

  impl Instruction {
    pub fn load(filename: &str) -> Vec<Instruction> {
      let mut f = File::open(filename).expect("Failed to open file");
      let mut s = String::new();
      f.read_to_string(&mut s).expect("Failed to read file");

      s.lines()
        .map(|line| Instruction::from_line(line))
        .collect()
    }

    pub fn from_line(line: &str) -> Instruction {
      let s = line.to_string();
      let parts : Vec<&str> = s.split_whitespace().collect();
      let operator = if parts[1] == "inc" { Operator::IncOperator } else { Operator::DecOperator };
      let comp = match parts[5] {
        "==" => eq,
        "!=" => neq,
        ">"  => gt,
        ">=" => gte,
        "<"  => lt,
        "<=" => lte,
        _ => panic!("Invalid comparator")
      };

      Instruction {
        register:  parts[0].to_string(),
        operator:  operator,
        value:     parts[2].parse::<i32>().expect("Invalid value"),
        condition: Condition {
          register: parts[4].to_string(),
          comp:     comp,
          value:    parts[6].parse::<i32>().expect("Invalid value")
        }
      }
    }
  }

  impl Computer {

    pub fn new() -> Computer {
      Computer { registers: HashMap::new(), max_value: 0 }
    }

    pub fn register<S: Into<String>>(&self, name: S) -> i32 {
      self.registers.get(&name.into()).cloned().unwrap_or(0)
    }

    pub fn increment(&mut self, register: String, value: i32) {
      if self.registers.contains_key(&register) {
        let v = self.registers.get_mut(&register).unwrap();
        *v += value;
        if *v > self.max_value { self.max_value = *v }
      } else {
        self.registers.insert(register, value);
        if value > self.max_value { self.max_value = value }
      }
    }

    pub fn process(&mut self, instructions: &Vec<Instruction>) {
      for i in instructions {
        let c = &i.condition;
        let v = self.register(c.register.clone());
        if (c.comp)(v, c.value) {
          match i.operator {
            Operator::IncOperator => { self.increment(i.register.clone(), i.value) },
            Operator::DecOperator => { self.increment(i.register.clone(), -i.value)}
          }
        }
      }
    }
  }

}

#[cfg(test)]
mod tests {
  use day8::*;

  #[test]
  fn it_works() {
    let instr = Instruction::load("./input.txt");
    let mut computer = Computer::new();

    computer.process(&instr);

    let (r, &value) = computer.registers.iter().max_by_key(|&(name, value)| value).unwrap();

    assert_eq!(value, 4877);

    assert_eq!(computer.max_value, 5471);
  }
}
