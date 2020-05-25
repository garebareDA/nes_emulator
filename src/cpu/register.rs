#[derive(Debug, Clone)]
struct Status {
  negative: bool,
  overflow: bool,
  reserved: bool,
  break_mode: bool,
  decimal_mode: bool,
  interrupt: bool,
  zero: bool,
  carry: bool,
}

#[allow(non_snake_case)] #[derive(Debug, Clone)]
pub struct Registers {
  A: u8,
  X: u8,
  Y: u8,
  SP: u8,
  PC: u16,
  P: Status,
}

impl Registers {
  pub fn new() -> Registers {
    Registers {
      A: 0,
      X: 0,
      Y: 0,
      PC: 0x8000,
      SP: 0xFD,
      P: Status {
        negative: false,
        overflow: false,
        reserved: true,
        break_mode: true,
        decimal_mode: false,
        interrupt: true,
        zero: false,
        carry: false,
      },
    }
  }

  pub fn set_interrupt(&mut self, i:bool) {
    self.P.interrupt = i;
  }

  pub fn set_PC(&mut self, pc: u16) {
    self.PC = pc;
  }

  pub fn set_break(&mut self, breaks: bool) {
    self.P.break_mode = breaks;
  }

  pub fn get_P(&self) -> u8 {
    bool_to_u8(self.P.negative) << 7
      | bool_to_u8(self.P.overflow) << 6
      | bool_to_u8(self.P.reserved) << 5
      | bool_to_u8(self.P.break_mode) << 4
      | bool_to_u8(self.P.decimal_mode) << 3
      | bool_to_u8(self.P.interrupt) << 2
      | bool_to_u8(self.P.zero) << 1
      | bool_to_u8(self.P.carry) as u8
  }

  pub fn get_interrupt(&self) -> bool{
    return self.P.interrupt;
  }

  pub fn get_PC(&self) -> u16 {
    return self.PC;
  }

  pub fn get_SP(&self) -> u8 {
    return self.SP;
  }

  pub fn dec_SP(&mut self) {
    self.SP -= 1;
  }

  pub fn inc_PC(&mut self) {
    self.PC += 1;
  }
}

fn bool_to_u8(v: bool) -> u8 {
  if v {
    1
  } else {
    0
  }
}