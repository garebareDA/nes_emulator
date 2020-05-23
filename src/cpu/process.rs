use super::register::Registers;
use super::super::bus::Bus;

pub fn reset(register: &mut Registers, bus: &mut Bus) {
  let pc = bus.read_word(0xFFFC);
  register.set_PC(pc);
  register.dec_SP();
}

pub fn brk(register: &mut Registers, bus:&mut Bus) {
  
}

pub fn process_nmi(register: &mut Registers, bus: &mut Bus) {
  register.set_break(false);
  push((register.get_PC() >> 8) as u8, register, bus);
  push(register.get_PC() as u8, register, bus);
  push_status(register, bus);
  let next = bus.read_word(0xFFFA);
  register.set_PC(next);
}

fn push_status(register: &mut Registers, bus: &mut Bus){
  let status = register.get_P();
  push(status, register, bus);
}

fn push(data: u8, register: &mut Registers, bus: &mut Bus){
  let addr = register.get_SP() as u16;
  bus.write((addr | 0x0100), data);
  register.dec_SP();
}