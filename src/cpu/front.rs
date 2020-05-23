use super::register::Registers;
use super::super::bus;

pub fn reset(register: &mut Registers, bus: &mut bus::Bus){
  let pc = bus.read_word(0xFFFC);
  register.set_PC(pc);
  register.dec_SP();
}