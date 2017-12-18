mod day18 {
  use std::collections::HashMap;
  use std::sync::mpsc::{Sender, Receiver};
  use std::sync::mpsc;
  use std::thread;

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
    pid: i64,
    registers: HashMap<char, i64>,
    ip: isize,
    sender:   Sender<i64>,
    receiver: Receiver<i64>,
    send_count: usize
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

    pub fn new(pid: i64, tx: Sender<i64>, rx: Receiver<i64>) -> Machine {
      let mut m = Machine {
        pid: pid,
        registers: HashMap::new(),
        ip: 0,
        sender: tx,
        receiver: rx,
        send_count: 0
      };

      m.set('p', pid);

      m
    }

    pub fn execute(&mut self, instructions: &[Instruction]) {
      let size = instructions.len() as isize;

      while self.ip >= 0 && self.ip < size {
        let instr = &instructions[self.ip as usize];
        println!("pid {}, ip = {} instr = {:?}, (count = {})", self.pid, self.ip, instr, self.send_count);
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
          let v = self.get_operand(&target);
          self.send_count += 1;
          println!("pid {} sent {}", self.pid, v);
          self.sender.send(v).unwrap();
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
        &Instruction::Rcv(ref target) => {
          if let &Operand::Register(r) = target {
            let v = self.receiver.recv().unwrap();
            println!("pid {} received {}", self.pid, v);
            self.set(r, v);
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
  use std::sync::mpsc::{Sender, Receiver};
  use std::sync::mpsc;
  use std::thread;

  #[test]
  fn test_p2() {

    let (t1x, r1x) : (Sender<i64>, Receiver<i64>) = mpsc::channel();
    let (t2x, r2x) : (Sender<i64>, Receiver<i64>) = mpsc::channel();

    let mut m1 = Machine::new(0, t1x, r2x);
    let mut m2 = Machine::new(1, t2x, r1x);
    let i = Machine::load(INPUT);
    let i2 = Machine::load(INPUT);

    let t = thread::spawn(move ||{
      m1.execute(&i);
    });

    let t2 = thread::spawn(move ||{
      m2.execute(&i2);
    });


    t.join();
    t2.join();

  }
}
