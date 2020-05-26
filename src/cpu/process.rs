use super::super::bus::Bus;
use super::super::helper::*;
use super::register::Registers;

pub fn adc(opeland: u16, register: &mut Registers, bus: &mut Bus, mode: &str) {
  let fetched = if mode == "immediate" {
    bus.read(opeland) as u16
  } else {
    opeland
  };
  let tmp = register.get_A() as u16 + fetched as u16 + bool_to_u8(register.get_carry()) as u16;
  let result = (tmp & 0xff) as u8;

  let is_carry = tmp > 0x00ffu16;
  let is_zero = result == 0;
  let is_negative = (result & 0x80) == 0x80;
  let is_overfloaw = ((register.get_A() ^ result) & (opeland as u8 ^ result) & 0x80) == 0x80;

  register.set_carry(is_carry);
  register.set_zero(is_zero);
  register.set_negative(is_negative);
  register.set_overflow(is_overfloaw);
  register.set_A(result);
}

pub fn sbc(opeland: u16, register: &mut Registers, bus: &mut Bus, mode: &str) {
  let fetched = if mode == "immediate" {
    bus.read(opeland) as u16
  } else {
    opeland
  };
  let result = register.get_A() as u16 - fetched as u16 - bool_to_u8(!register.get_carry()) as u16;
  let acc = register.get_A();

  let is_carry = result > 0xFF;
  let is_zero = result == 0;
  let is_negative = (result & 0x80) == 0x80;
  let is_overfloaw =
    (acc ^ (fetched as u8)) & 0x80 == 0x80 && ((acc ^ result as u8) & 0x80) == 0x80;

  register.set_carry(is_carry);
  register.set_zero(is_zero);
  register.set_negative(is_negative);
  register.set_overflow(is_overfloaw);
  register.set_A(result as u8);
}

pub fn and(opeland: u16, register: &mut Registers, bus: &mut Bus, mode: &str) {
  let fetched = if mode == "immediate" {
    bus.read(opeland) as u16
  } else {
    opeland
  };
  let result = register.get_A() as u16 & fetched;
  let is_zero = result == 0;
  let is_negative = (result & 0x80) == 0x80;

  register.set_zero(is_zero);
  register.set_negative(is_negative);
  register.set_A(result as u8);
}

pub fn ora(opeland: u16, register: &mut Registers, bus: &mut Bus, mode: &str) {
  let fetched = if mode == "immediate" {
    bus.read(opeland) as u16
  } else {
    opeland
  };
  let result = register.get_A() as u16 | fetched;

  let is_zero = result == 0;
  let is_negative = (result & 0x80) == 0x80;

  register.set_zero(is_zero);
  register.set_negative(is_negative);
  register.set_A(result as u8);
}

pub fn eor(opeland: u16, register: &mut Registers, bus: &mut Bus, mode: &str) {
  let fetched = if mode == "immediate" {
    bus.read(opeland) as u16
  } else {
    opeland
  };
  let result = register.get_A() as u16 ^ fetched;

  let is_zero = result == 0;
  let is_negative = (result & 0x80) == 0x80;

  register.set_zero(is_zero);
  register.set_negative(is_negative);
  register.set_A(result as u8);
}

pub fn asl(opeland: u16, register: &mut Registers, bus: &mut Bus, mode: &str) {
  let fetched = if mode == "accumulator" {
    register.get_A()
  } else {
    bus.read(opeland)
  };
  let shifted = (fetched << 1) as u8;

  let is_carry = (shifted & 0x80) == 0x80;
  let is_zero = shifted == 0;
  let is_negative = (shifted & 0x80) == 0x80;

  register.set_carry(is_carry);
  register.set_zero(is_zero);
  register.set_negative(is_negative);

  if mode == "accumulator" {
    register.set_A(shifted);
  } else {
    bus.write(opeland, shifted);
  }
}

pub fn lsr(opeland: u16, register: &mut Registers, bus: &mut Bus, mode: &str) {
  let fetched = if mode == "accumulator" {
    register.get_A()
  } else {
    bus.read(opeland)
  };
  let shifted = (fetched >> 1) as u8;

  let is_carry = (shifted & 0x01) == 0x01;
  let is_zero = shifted == 0;
  let is_negative = (shifted & 0x80) == 0x80;

  register.set_carry(is_carry);
  register.set_zero(is_zero);
  register.set_negative(is_negative);

  if mode == "accumulator" {
    register.set_A(shifted);
  } else {
    bus.write(opeland, shifted);
  }
}

pub fn rol(opeland: u16, register: &mut Registers, bus: &mut Bus, mode: &str) {
  let fetched = if mode == "accumulator" {
    register.get_A()
  } else {
    bus.read(opeland)
  };

  let shifted = fetched << 1 | if register.get_carry() { 0x01 } else { 0x00 };

  let is_carry = (shifted & 0x01) == 0x01;
  let is_zero = shifted == 0;
  let is_negative = (shifted & 0x80) == 0x80;

  register.set_carry(is_carry);
  register.set_zero(is_zero);
  register.set_negative(is_negative);

  if mode == "accumulator" {
    register.set_A(shifted);
  } else {
    bus.write(opeland, shifted);
  }
}

pub fn ror(opeland: u16, register: &mut Registers, bus: &mut Bus, mode: &str) {
  let fetched = if mode == "accumulator" {
    register.get_A()
  } else {
    bus.read(opeland)
  };

  let shifted = fetched << 1 | if register.get_carry() { 0x80 } else { 0x00 };

  let is_carry = (shifted & 0x01) == 0x01;
  let is_zero = shifted == 0;
  let is_negative = (shifted & 0x80) == 0x80;

  register.set_carry(is_carry);
  register.set_zero(is_zero);
  register.set_negative(is_negative);

  if mode == "accumulator" {
    register.set_A(shifted);
  } else {
    bus.write(opeland, shifted);
  }
}

pub fn bcc(opeland: u16, register: &mut Registers) {
  if !register.get_carry() {
    register.set_PC(opeland);
  }
}

pub fn bcs(opeland: u16, register: &mut Registers) {
  if register.get_carry() {
    register.set_PC(opeland);
  }
}

pub fn beq(opeland: u16, register: &mut Registers) {
  if register.get_zero() {
    register.set_PC(opeland);
  }
}

pub fn bne(opeland: u16, register: &mut Registers) {
  if !register.get_zero() {
    register.set_PC(opeland);
  }
}

pub fn bvc(opeland: u16, register: &mut Registers) {
  if !register.get_overflow() {
    register.set_PC(opeland);
  }
}

pub fn bvs(opeland: u16, register: &mut Registers) {
  if register.get_overflow() {
    register.set_PC(opeland);
  }
}

pub fn bpl(opeland: u16, register: &mut Registers) {
  if !register.get_negative() {
    register.set_PC(opeland);
  }
}

pub fn bmi(opeland: u16, register: &mut Registers) {
  if register.get_negative() {
    register.set_PC(opeland);
  }
}

pub fn bit(opeland: u16, register: &mut Registers, bus: &mut Bus) {
  let fetched = bus.read(opeland);
  let and = register.get_A() & fetched;

  let is_zero = and == 0;
  let is_negative = (and & 0x80) == 0x80;
  let is_overfloaw = (and & 0x40) == 0x40;

  register.set_zero(is_zero);
  register.set_negative(is_negative);
  register.set_overflow(is_overfloaw);
}

pub fn jmp(opeland: u16, register: &mut Registers) {
  register.set_PC(opeland);
}

pub fn jsr(opeland: u16, register: &mut Registers, bus: &mut Bus) {
  let pc = register.get_PC();
  push((pc >> 8) as u8, register, bus);
  push(pc as u8, register, bus);
  register.set_PC(opeland);
}

pub fn rts(register: &mut Registers, bus: &mut Bus) {
  let lower = pop(register, bus) as u16;
  let upper = pop(register, bus) as u16;
  register.set_PC(upper << 8 | lower);
  register.inc_PC();
}

pub fn cmp(operand:u16, register: &mut Registers, bus: &mut Bus, mode:&str){
  let fetchd = if mode == "immediate" {
    bus.read(operand) as u16
  }else{
    operand
  };

  let result = register.get_A() as u16 - fetchd;

  let is_carry = register.get_A() as u16 >= fetchd;
  let is_zero = result == 0;
  let is_negative = (result & 0x80) == 0x80;

  register.set_carry(is_carry);
  register.set_zero(is_zero);
  register.set_negative(is_negative);
}

pub fn cpx(operand:u16, register: &mut Registers, bus: &mut Bus, mode:&str){
  let fetchd = if mode == "immediate" {
    bus.read(operand) as u16
  }else{
    operand
  };

  let result = register.get_X() as u16 - fetchd;

  let is_carry = register.get_X() as u16 >= fetchd;
  let is_zero = result == 0;
  let is_negative = (result & 0x80) == 0x80;

  register.set_carry(is_carry);
  register.set_zero(is_zero);
  register.set_negative(is_negative);
}

pub fn cpy(operand:u16, register: &mut Registers, bus: &mut Bus, mode:&str){
  let fetchd = if mode == "immediate" {
    bus.read(operand) as u16
  }else{
    operand
  };

  let result = register.get_Y() as u16 - fetchd;

  let is_carry = register.get_Y() as u16 >= fetchd;
  let is_zero = result == 0;
  let is_negative = (result & 0x80) == 0x80;

  register.set_carry(is_carry);
  register.set_zero(is_zero);
  register.set_negative(is_negative);
}

pub fn inc(operand:u16, register: &mut Registers, bus: &mut Bus,) {
  let fetched = bus.read(operand) + 1;

  let is_zero = fetched == 0;
  let is_negative = (fetched & 0x80) == 0x80;

  register.set_zero(is_zero);
  register.set_negative(is_negative);
  bus.write(operand, fetched);
}

pub fn dec(operand:u16, register: &mut Registers, bus: &mut Bus) {
  let fetched = bus.read(operand) - 1;

  let is_zero = fetched == 0;
  let is_negative = (fetched & 0x80) == 0x80;

  register.set_zero(is_zero);
  register.set_negative(is_negative);
  bus.write(operand, fetched);
}

pub fn inx(register: &mut Registers) {
  let result = register.get_X() + 1;

  let is_zero = result == 0;
  let is_negative = (result & 0x80) == 0x80;

  register.set_zero(is_zero);
  register.set_negative(is_negative);
  register.set_X(result);
}

pub fn dex(register: &mut Registers) {
  let result = register.get_X() - 1;

  let is_zero = result == 0;
  let is_negative = (result & 0x80) == 0x80;

  register.set_zero(is_zero);
  register.set_negative(is_negative);
  register.set_X(result);
}

pub fn iny(register: &mut Registers) {
  let result = register.get_Y() + 1;

  let is_zero = result == 0;
  let is_negative = (result & 0x80) == 0x80;

  register.set_zero(is_zero);
  register.set_negative(is_negative);
  register.set_Y(result);
}

pub fn dey(register: &mut Registers) {
  let result = register.get_Y() - 1;

  let is_zero = result == 0;
  let is_negative = (result & 0x80) == 0x80;

  register.set_zero(is_zero);
  register.set_negative(is_negative);
  register.set_Y(result);
}

pub fn nop(){
  return;
}

pub fn reset(register: &mut Registers, bus: &mut Bus) {
  let pc = bus.read_word(0xFFFC);
  register.set_interrupt(true);
  register.set_PC(pc);
}

pub fn nmi(register: &mut Registers, bus: &mut Bus) {
  register.set_break(false);
  push((register.get_PC() >> 8) as u8, register, bus);
  push(register.get_PC() as u8, register, bus);
  push_status(register, bus);
  register.set_interrupt(true);
}

pub fn irq(register: &mut Registers, bus: &mut Bus) {
  let interrupt = register.get_interrupt();
  if interrupt {
    return;
  }
  register.set_break(false);
  register.inc_PC();
  push((register.get_PC() >> 8) as u8, register, bus);
  push(register.get_PC() as u8, register, bus);
  push_status(register, bus);
  let next = bus.read_word(0xFFFE);
  register.set_PC(next);
  register.set_interrupt(false);
}

pub fn brk(register: &mut Registers, bus: &mut Bus) {
  let interrupt = register.get_interrupt();
  if !interrupt {
    return;
  }
  register.set_break(true);
  register.inc_PC();
  push((register.get_PC() >> 8) as u8, register, bus);
  push(register.get_PC() as u8, register, bus);
  push_status(register, bus);
  let next = bus.read_word(0xFFFE);
  register.set_PC(next);
}

fn push_status(register: &mut Registers, bus: &mut Bus) {
  let status = register.get_P();
  push(status, register, bus);
}

fn push(data: u8, register: &mut Registers, bus: &mut Bus) {
  let addr = register.get_SP() as u16;
  bus.write((addr | 0x0100), data);
  register.dec_SP();
}

fn pop(register: &mut Registers, bus: &mut Bus) -> u8{
  register.inc_PC();
  let addr = 0x0100 | register.get_SP() as u16;
  bus.read(addr)
}