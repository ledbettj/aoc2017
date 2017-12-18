mod day18 {
  use std::collections::HashMap;

  #[derive(Debug)]
  pub enum Operand {
    Register(char),
    Integer(i64)
  }

  #[derive(Debug)]
  pub enum Instruction {
    Snd(Operand),
    Set(Operand, Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Mod(Operand, Operand),
    Rcv(Operand),
    Jgz(Operand, Operand)
  }

  pub struct Machine {
    registers: HashMap<char, i64>,
    ip: isize,
    sound: i64,
    pub recovered: Vec<i64>
  }

  impl Machine {

    fn parse_operand(s: &str) -> Operand {
      let ch = s.chars().last().unwrap();

      match s.parse::<i64>() {
        Ok(i)  => Operand::Integer(i),
        Err(e) => Operand::Register(ch)
      }
    }

    pub fn load(input: &str) -> Vec<Instruction> {
      input.lines().map(|line| {
        let parts : Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
          "snd" => Instruction::Snd(Machine::parse_operand(parts[1])),
          "set" => Instruction::Set(Machine::parse_operand(parts[1]), Machine::parse_operand(parts[2])),
          "add" => Instruction::Add(Machine::parse_operand(parts[1]), Machine::parse_operand(parts[2])),
          "mul" => Instruction::Mul(Machine::parse_operand(parts[1]), Machine::parse_operand(parts[2])),
          "mod" => Instruction::Mod(Machine::parse_operand(parts[1]), Machine::parse_operand(parts[2])),
          "rcv" => Instruction::Rcv(Machine::parse_operand(parts[1])),
          "jgz" => Instruction::Jgz(Machine::parse_operand(parts[1]), Machine::parse_operand(parts[2])),
          x     => panic!("invalid instruction: {}", x)

        }
      }).collect()

    }

    pub fn new() -> Machine {
      Machine {
        registers: HashMap::new(),
        ip: 0,
        sound: 0,
        recovered: Vec::new()
      }
    }

    pub fn execute(&mut self, instructions: &[Instruction]) {
      let size = instructions.len() as isize;

      while self.ip >= 0 && self.ip < size {
        let instr = &instructions[self.ip as usize];
        println!("ip = {} instr = {:?}", self.ip, instr);
        self.step(&instr);
      }
    }

    pub fn get(&mut self, register: char) -> i64 {
      *self.registers.entry(register).or_insert(0)
    }

    pub fn set(&mut self, register: char, value: i64) {
      self.registers.insert(register, value);
    }

    pub fn get_operand(&mut self, operand: &Operand) -> i64 {
      match operand {
        &Operand::Integer(i)  => i,
        &Operand::Register(r) => self.get(r)
      }
    }

    pub fn step(&mut self, instruction: &Instruction) {
      match instruction {
        &Instruction::Snd(ref target) => {
          self.sound = self.get_operand(&target);
          self.ip += 1;
        }
        &Instruction::Set(ref target, ref value) => {
          let v = self.get_operand(&value);
          if let &Operand::Register(r) = target {
            self.set(r, v)
          }
          self.ip += 1;
        },
        &Instruction::Add(ref target, ref value) => {
          let v = self.get_operand(&value);
          if let &Operand::Register(r) = target {
            let i = self.get(r);
            self.set(r, i + v)
          }
          self.ip += 1;
        },
        &Instruction::Mul(ref target, ref value) => {
          let v = self.get_operand(&value);
          if let &Operand::Register(r) = target {
            let i = self.get(r);
            println!("mul {} {}", i, v);
            self.set(r, i * v)
          }
          self.ip += 1;
        },
        &Instruction::Mod(ref target, ref value) => {
          let v = self.get_operand(&value);
          if let &Operand::Register(r) = target {
            let i = self.get(r);
            self.set(r, i % v)
          }
          self.ip += 1;
        },
        &Instruction::Rcv(ref check) => {
          let v = self.get_operand(&check);
          if v != 0 {
            self.recovered.push(self.sound);
            // HACK: end this madness
            self.ip = -100;
          }
          self.ip += 1;
        },
        &Instruction::Jgz(ref check, ref offset) => {
          let c = self.get_operand(&check);
          let v = self.get_operand(&offset);
          if c > 0 {
            self.ip += v as isize;
          } else {
            self.ip += 1;
          }
        }
      }
    }
  }

}

fn main() {
  println!("Hello, world!");
}


#[cfg(test)]
mod tests {
  const INPUT : &str = include_str!("../input.txt");
  use day18::*;

  #[test]
  fn test_example() {
    let s = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

    let mut m = Machine::new();
    let i = Machine::load(s);

    m.execute(&i);

    assert_eq!(m.recovered[0], 4);
  }
  
  #[test]
  fn test_p1() {
    let mut m = Machine::new();
    let i = Machine::load(INPUT);

    m.execute(&i);

    assert_eq!(m.recovered[0], 2951);
  }
}
