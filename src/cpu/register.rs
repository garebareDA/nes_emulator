use super::super::helper::*;

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

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
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

  pub fn set_A(&mut self, a: u8) {
    self.A = a;
  }

  pub fn set_X(&mut self, x:u8) {
    self.X = x;
  }

  pub fn set_Y(&mut self, y:u8) {
    self.Y = y;
  }

  pub fn set_SP(&mut self, sp:u8){
    self.SP = sp;
  }

  pub fn set_P(&mut self, v: u8) -> &mut Self {
    self.P.negative = v & 0x80 == 0x80;
    self.P.overflow = v & 0x40 == 0x40;
    self.P.reserved = v & 0x20 == 0x20;
    self.P.break_mode = v & 0x10 == 0x10;
    self.P.decimal_mode = v & 0x08 == 0x08;
    self.P.interrupt = v & 0x04 == 0x04;
    self.P.zero = v & 0x02 == 0x02;
    self.P.carry = v & 0x01 == 0x01;
    self
}

  pub fn set_decimal(&mut self, d: bool) {
    self.P.decimal_mode = d;
  }

  pub fn set_interrupt(&mut self, i: bool) {
    self.P.interrupt = i;
  }

  pub fn set_PC(&mut self, pc: u16) {
    self.PC = pc;
  }

  pub fn set_negative(&mut self, nega: bool) {
    self.P.negative = nega
  }

  pub fn set_overflow(&mut self, over: bool) {
    self.P.overflow = over;
  }

  pub fn set_break(&mut self, breaks: bool) {
    self.P.break_mode = breaks;
  }

  pub fn set_zero(&mut self, zero: bool) {
    self.P.zero = zero;
  }

  pub fn set_carry(&mut self, carry: bool) {
    self.P.carry = carry;
  }

  pub fn get_A(&self) -> u8 {
    self.A
  }

  pub fn get_X(&self) -> u8 {
    self.X
  }

  pub fn get_Y(&self) -> u8 {
    self.Y
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

  pub fn get_interrupt(&self) -> bool {
    return self.P.interrupt;
  }

  pub fn get_PC(&self) -> u16 {
    return self.PC;
  }

  pub fn get_SP(&self) -> u8 {
    return self.SP;
  }

  pub fn get_negative(&self) -> bool {
    self.P.negative
  }

  pub fn get_overflow(&self) -> bool {
    self.P.overflow
  }

  pub fn get_zero(&self) -> bool {
    self.P.zero
  }

  pub fn get_carry(&self) -> bool {
    self.P.carry
  }

  pub fn inc_SP(&mut self) {
    self.SP += 1;
  }

  pub fn dec_SP(&mut self) {
    self.SP -= 1;
  }

  pub fn inc_PC(&mut self) {
    self.PC += 1;
  }
}
