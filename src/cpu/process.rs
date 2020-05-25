use super::super::helper:: *;
use super::super::bus::Bus;
use super::register::Registers;

pub fn adc(opeland:u16, register: &mut Registers, bus: &mut Bus) {
  let fetchd = bus.read(opeland);
  let tmp = register.get_A() as u16 + fetchd as u16 +bool_to_u8(register.get_carry())  as u16;
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

pub fn sbc(opeland:u16, register: &mut Registers, bus: &mut Bus) {
  let fetched = bus.read(opeland);
  let result = register.get_A() as u16 - fetched as u16 - bool_to_u8(!register.get_carry())  as u16;
  let acc = register.get_A();

  let is_carry = result > 0xFF;
  let is_zero = result == 0;
  let is_negative = (result & 0x80) == 0x80;
  let is_overfloaw = (acc ^ (fetched as u8)) & 0x80 == 0x80 && ((acc ^ result as u8) & 0x80) == 0x80;

  register.set_carry(is_carry);
  register.set_zero(is_zero);
  register.set_negative(is_negative);
  register.set_overflow(is_overfloaw);
  register.set_A(result as u8);
}

pub fn and(opeland:u16, register: &mut Registers, bus: &mut Bus) {
  
}

pub fn reset(register: &mut Registers, bus: &mut Bus) {
  let pc = bus.read_word(0xFFFC);
  register.set_interrupt(true);
  register.set_PC(pc);
}

pub fn process_nmi(register: &mut Registers, bus: &mut Bus) {
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
