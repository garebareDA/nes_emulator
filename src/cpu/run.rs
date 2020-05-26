use super::super::bus::Bus;
use super::register::Registers;

enum Opecode {
  ADC,
  SBC,
  AND,
  EOR,
  ORA,
  ASL,
  LSR,
  ROL,
  ROR,
  INC,
  INX,
  INY,
  DEC,
  DEX,
  DEY,
  LDA,
  LDX,
  LDY,
  STA,
  STX,
  STY,
  SEC,
  SED,
  SEI,
  CLC,
  CLD,
  CLI,
  CLV,
  CMP,
  CPX,
  CPY,
  JMP,
  JSR,
  RTI,
  RTS,
  BCC,
  BCS,
  BEQ,
  BMI,
  BNE,
  BPL,
  BVC,
  BVS,
  PHA,
  PHP,
  PLA,
  PLP,
  TAX,
  TAY,
  TSX,
  TXA,
  TXS,
  TYA,
  BRK,
  BIT,
  NOP,
  ALR,
  ANC,
  ARR,
  AXS,
  LAX,
  SAX,
  DCP,
  ISC,
  RLA,
  RRA,
  SLO,
  SRE,
  SKB,
  IGN,
}

pub enum Addressing {
  Immediate,
  ZeroPage,
  Relative,
  Implied,
  Absolute,
  Accumulator,
  ZeroPageX,
  ZeroPageY,
  AbsoluteX,
  AbsoluteY,
  IndirectX,
  IndirectY,
  IndirectAbsolute,
}

pub fn run(register:&mut Registers, bus:&mut Bus) {
  let opecode = fetch(register, bus);
  
}

fn fetchOpeland(addressing:&Addressing, register:&mut Registers, bus:&mut Bus) -> u16{
  match addressing {
    Addressing::Accumulator => {0x0000},
    Addressing::Implied => {0x0000},
    Addressing::Immediate => fetch(register, bus) as u16,
    Addressing::Relative => {
      let addr = fetch(register, bus) as u16;
      if addr < 0x80 {
        addr + register.get_PC()
    } else {
        addr + register.get_PC() - 256
    }
    },
    Addressing::ZeroPage => fetch(register, bus) as u16,
    Addressing::ZeroPageX => {
      let addr = fetch(register, bus);
      let result = (addr + register.get_X()) & 0xFF;
      result as u16
    },
    Addressing::ZeroPageY=> {
      let addr = fetch(register, bus);
      let result = (addr + register.get_Y()) & 0xFF;
      result as u16
    },
    Addressing::Absolute => {
      let addr = fetch_word(register, bus);
      addr
    },
    Addressing::AbsoluteX => {
      let addr = fetch_word(register, bus);
      let result = (addr + register.get_X() as u16) & 0xFFFF;
      result
    },
    Addressing::AbsoluteY => {
      let addr = fetch_word(register, bus);
      let result = (addr + register.get_Y() as u16) & 0xFFFF;
      result
    },
    Addressing::IndirectX => {
      let base_addr = (fetch(register, bus) + register.get_X()) & 0xFF;
      let addr = bus.read(base_addr as u16) + (bus.read((base_addr as u16 + 1) & 0xFF) << 8);
      let result = addr & 0xFF;
      result as u16
    },
    Addressing::IndirectY => {
      let addr_or_data = fetch(register, bus);
      let base_addr = bus.read(addr_or_data as u16) + (bus.read(addr_or_data as u16 + 1) & 0xFF) << 8;
      let addr = (base_addr + register.get_Y()) & 0xFF;
      addr as u16
    }
    Addressing::IndirectAbsolute => {
      let addr_or_data = fetch_word(register, bus);
      let addr = bus.read(addr_or_data) + (bus.read((addr_or_data & 0xFF00) | (((addr_or_data & 0xFF) + 1) & 0xFF)) << 8);
      let result = addr & 0xFF;
      result as u16
    }
  }
}

fn from_code(code:u8) -> (Opecode, Addressing) {
  match code {
    0x69 => (Opecode::ADC, Addressing::Immediate),
    0x65 => (Opecode::ADC, Addressing::ZeroPage),
    0x75 => (Opecode::ADC, Addressing::ZeroPageX),
    0x6D => (Opecode::ADC, Addressing::Absolute),
    0x7D => (Opecode::ADC, Addressing::AbsoluteX),
    0x79 => (Opecode::ADC, Addressing::AbsoluteY),
    0x61 => (Opecode::ADC, Addressing::IndirectX),
    0x71 => (Opecode::ADC, Addressing::IndirectY),

    0xE9 => (Opecode::SBC, Addressing::Immediate),
    0xE5 => (Opecode::SBC, Addressing::ZeroPage),
    0xF5 => (Opecode::SBC, Addressing::ZeroPageX),
    0xED => (Opecode::SBC, Addressing::Absolute),
    0xFD => (Opecode::SBC, Addressing::AbsoluteX),
    0xF9 => (Opecode::SBC, Addressing::AbsoluteY),
    0xE1 => (Opecode::SBC, Addressing::IndirectX),
    0xF1 => (Opecode::SBC, Addressing::IndirectY),

    0x29 => (Opecode::AND, Addressing::Immediate),
    0x25 => (Opecode::AND, Addressing::ZeroPage),
    0x35 => (Opecode::AND, Addressing::ZeroPageX),
    0x2D => (Opecode::AND, Addressing::Absolute),
    0x3D => (Opecode::AND, Addressing::AbsoluteX),
    0x39 => (Opecode::AND, Addressing::AbsoluteY),
    0x21 => (Opecode::AND, Addressing::IndirectX),
    0x31 => (Opecode::AND, Addressing::IndirectY),

    0x09 => (Opecode::ORA, Addressing::Immediate),
    0x05 => (Opecode::ORA, Addressing::ZeroPage),
    0x15 => (Opecode::ORA, Addressing::ZeroPageX),
    0x0D => (Opecode::ORA, Addressing::Absolute),
    0x1D => (Opecode::ORA, Addressing::AbsoluteX),
    0x19 => (Opecode::ORA, Addressing::AbsoluteY),
    0x01 => (Opecode::ORA, Addressing::IndirectX),
    0x11 => (Opecode::ORA, Addressing::IndirectY),

    
    _ => {}
  }
}

fn fetch(register:&mut Registers, bus: &mut Bus) -> u8 {
  let code = bus.read(register.get_PC());
  register.inc_PC();
  code
}

fn fetch_word(register:&mut Registers, bus:&mut Bus) -> u16 {
  let lower = bus.read(register.get_PC()) as u16;
  register.inc_PC();
  let upper = bus.read(register.get_PC()) as u16;
  register.inc_PC();
  (upper << 8 | lower) as u16
}