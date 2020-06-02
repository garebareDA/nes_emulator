use super::super::bus::Bus;
use super::process;
use super::register::Registers;

#[derive(Debug)]
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

#[derive(Debug)]
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

pub fn run(register: &mut Registers, bus: &mut Bus) -> u8 {
  let code = fetch(register, bus);
  let (opecode, mode, cycle) = from_code(code);
  let opeland = fetch_opeland(&mode, register, bus);

  match opecode {
    Opecode::BRK => {
      process::brk(register, bus);
    }

    Opecode::ADC => match mode {
      Addressing::Immediate => process::adc(opeland, register, bus, "immediate"),
      _ => process::adc(opeland, register, bus, "non"),
    },

    Opecode::SBC => match mode {
      Addressing::Immediate => process::sbc(opeland, register, bus, "immediate"),
      _ => process::sbc(opeland, register, bus, "non"),
    },

    Opecode::AND => match mode {
      Addressing::Immediate => process::and(opeland, register, bus, "immediate"),
      _ => process::and(opeland, register, bus, "non"),
    },

    Opecode::ORA => match mode {
      Addressing::Immediate => process::ora(opeland, register, bus, "immediate"),
      _ => process::ora(opeland, register, bus, "non"),
    },

    Opecode::EOR => match mode {
      Addressing::Immediate => process::eor(opeland, register, bus, "immediate"),
      _ => process::eor(opeland, register, bus, "non"),
    },

    Opecode::ASL => match mode {
      Addressing::Immediate => process::asl(opeland, register, bus, "immediate"),
      _ => process::asl(opeland, register, bus, "non"),
    },

    Opecode::LSR => match mode {
      Addressing::Immediate => process::lsr(opeland, register, bus, "immediate"),
      _ => process::lsr(opeland, register, bus, "non"),
    },

    Opecode::ROL => match mode {
      Addressing::Immediate => process::rol(opeland, register, bus, "immediate"),
      _ => process::rol(opeland, register, bus, "non"),
    },

    Opecode::ROR => match mode {
      Addressing::Immediate => process::ror(opeland, register, bus, "immediate"),
      _ => process::ror(opeland, register, bus, "non"),
    },

    Opecode::BCC => {
      process::bcc(opeland, register);
    }

    Opecode::BCS => {
      process::bcs(opeland, register);
    }

    Opecode::BEQ => {
      process::beq(opeland, register);
    }

    Opecode::BNE => {
      process::bne(opeland, register);
    }

    Opecode::BVC => {
      process::bvc(opeland, register);
    }

    Opecode::BVS => {
      process::bvs(opeland, register);
    }

    Opecode::BPL => {
      process::bpl(opeland, register);
    }

    Opecode::BMI => {
      process::bmi(opeland, register);
    }

    Opecode::BIT => {
      process::bit(opeland, register, bus);
    }

    Opecode::JMP => {
      process::jmp(opeland, register);
    }

    Opecode::JSR => {
      process::jsr(opeland, register, bus);
    }

    Opecode::RTS => {
      process::rts(register, bus);
    }

    Opecode::CMP => match mode {
      Addressing::Immediate => process::cmp(opeland, register, bus, "immediate"),
      _ => process::cmp(opeland, register, bus, "non"),
    },

    Opecode::CPX => match mode {
      Addressing::Immediate => process::cpx(opeland, register, bus, "immediate"),
      _ => process::cmp(opeland, register, bus, "non"),
    },

    Opecode::CPY => match mode {
      Addressing::Immediate => process::cpy(opeland, register, bus, "immediate"),
      _ => process::cpy(opeland, register, bus, "non"),
    },

    Opecode::INC => {
      process::inc(opeland, register, bus);
    }

    Opecode::DEC => {
      process::dec(opeland, register, bus);
    }

    Opecode::INX => {
      process::inx(register);
    }

    Opecode::DEX => {
      process::dex(register);
    }

    Opecode::INY => {
      process::iny(register);
    }

    Opecode::DEY => {
      process::dey(register);
    }

    Opecode::CLC => {
      process::clc(register);
    }

    Opecode::SEC => {
      process::sec(register);
    }

    Opecode::CLI => {
      process::cli(register);
    }

    Opecode::SEI => {
      process::sei(register);
    }

    Opecode::CLD => {
      process::cld(register);
    }

    Opecode::SED => {
      process::sed(register);
    }

    Opecode::CLV => {
      process::clv(register);
    }

    Opecode::LDA => match mode {
      Addressing::Immediate => process::lda(opeland, register, bus, "immediate"),
      _ => process::lda(opeland, register, bus, "non"),
    },

    Opecode::LDX => match mode {
      Addressing::Immediate => process::ldx(opeland, register, bus, "immediate"),
      _ => process::ldx(opeland, register, bus, "non"),
    },

    Opecode::LDY => match mode {
      Addressing::Immediate => process::ldy(opeland, register, bus, "immediate"),
      _ => process::ldy(opeland, register, bus, "non"),
    },

    Opecode::STA => {
      process::sta(opeland, register, bus);
    }

    Opecode::STX => {
      process::stx(opeland, register, bus);
    }

    Opecode::STY => {
      process::sty(opeland, register, bus);
    }

    Opecode::TAX => {
      process::tax(register);
    }

    Opecode::TXA => {
      process::txa(register);
    }

    Opecode::TAY => {
      process::tay(register);
    }

    Opecode::TYA => {
      process::tya(register);
    }

    Opecode::TSX => {
      process::tsx(register);
    }

    Opecode::TXS => {
      process::txs(register);
    }

    Opecode::PHA => {
      process::pha(register, bus);
    }

    Opecode::PLA => {
      process::pla(register, bus);
    }

    Opecode::PHP => {
      process::php(register, bus);
    }

    Opecode::PLP => {
      process::plp(register, bus);
    }

    Opecode::NOP => process::nop(),

    _ => {}
  }

  return cycle;
}

fn fetch_opeland(addressing: &Addressing, register: &mut Registers, bus: &mut Bus) -> u16 {
  match addressing {
    Addressing::Accumulator => 0x0000,
    Addressing::Implied => 0x0000,
    Addressing::Immediate => fetch(register, bus) as u16,
    Addressing::Relative => {
      let addr = fetch(register, bus) as u16;
      if addr < 0x80 {
        addr + register.get_PC()
      } else {
        addr + register.get_PC() - 256
      }
    }
    Addressing::ZeroPage => fetch(register, bus) as u16,
    Addressing::ZeroPageX => {
      let addr = fetch(register, bus);
      let result = (addr + register.get_X()) & 0xFF;
      result as u16
    }
    Addressing::ZeroPageY => {
      let addr = fetch(register, bus);
      let result = (addr + register.get_Y()) & 0xFF;
      result as u16
    }
    Addressing::Absolute => {
      let addr = fetch_word(register, bus);
      addr
    }
    Addressing::AbsoluteX => {
      let addr = fetch_word(register, bus);
      let result = (addr + register.get_X() as u16) & 0xFFFF;
      result
    }
    Addressing::AbsoluteY => {
      let addr = fetch_word(register, bus);
      let result = (addr + register.get_Y() as u16) & 0xFFFF;
      result
    }
    Addressing::IndirectX => {
      let base_addr = (fetch(register, bus) + register.get_X()) & 0xFF;
      let addr = bus.read(base_addr as u16) + (bus.read((base_addr as u16 + 1) & 0xFF) << 8);
      let result = addr & 0xFF;
      result as u16
    }
    Addressing::IndirectY => {
      let addr_or_data = fetch(register, bus);
      let base_addr =
        bus.read(addr_or_data as u16) + (bus.read(addr_or_data as u16 + 1) & 0xFF) << 8;
      let addr = (base_addr + register.get_Y()) & 0xFF;
      addr as u16
    }
    Addressing::IndirectAbsolute => {
      let addr_or_data = fetch_word(register, bus);
      let addr = bus.read(addr_or_data)
        + (bus.read((addr_or_data & 0xFF00) | (((addr_or_data & 0xFF) + 1) & 0xFF)) << 8);
      let result = addr & 0xFF;
      result as u16
    }
  }
}

fn fetch(register: &mut Registers, bus: &mut Bus) -> u8 {
  let code = bus.read(register.get_PC());
  register.inc_PC();
  code
}

fn fetch_word(register: &mut Registers, bus: &mut Bus) -> u16 {
  let lower = bus.read(register.get_PC()) as u16;
  register.inc_PC();
  let upper = bus.read(register.get_PC()) as u16;
  register.inc_PC();
  (upper << 8 | lower) as u16
}

fn from_code(code: u8) -> (Opecode, Addressing, u8) {
  match code {
    0x69 => (Opecode::ADC, Addressing::Immediate, 2),
    0x65 => (Opecode::ADC, Addressing::ZeroPage, 3),
    0x75 => (Opecode::ADC, Addressing::ZeroPageX, 4),
    0x6D => (Opecode::ADC, Addressing::Absolute, 4),
    0x7D => (Opecode::ADC, Addressing::AbsoluteX, 4),
    0x79 => (Opecode::ADC, Addressing::AbsoluteY, 4),
    0x61 => (Opecode::ADC, Addressing::IndirectX, 6),
    0x71 => (Opecode::ADC, Addressing::IndirectY, 5),

    0xE9 => (Opecode::SBC, Addressing::Immediate, 2),
    0xE5 => (Opecode::SBC, Addressing::ZeroPage, 3),
    0xF5 => (Opecode::SBC, Addressing::ZeroPageX, 4),
    0xED => (Opecode::SBC, Addressing::Absolute, 4),
    0xFD => (Opecode::SBC, Addressing::AbsoluteX, 4),
    0xF9 => (Opecode::SBC, Addressing::AbsoluteY, 4),
    0xE1 => (Opecode::SBC, Addressing::IndirectX, 5),
    0xF1 => (Opecode::SBC, Addressing::IndirectY, 6),

    0x29 => (Opecode::AND, Addressing::Immediate, 2),
    0x25 => (Opecode::AND, Addressing::ZeroPage, 3),
    0x35 => (Opecode::AND, Addressing::ZeroPageX, 4),
    0x2D => (Opecode::AND, Addressing::Absolute, 4),
    0x3D => (Opecode::AND, Addressing::AbsoluteX, 4),
    0x39 => (Opecode::AND, Addressing::AbsoluteY, 4),
    0x21 => (Opecode::AND, Addressing::IndirectX, 5),
    0x31 => (Opecode::AND, Addressing::IndirectY, 6),

    0x09 => (Opecode::ORA, Addressing::Immediate, 2),
    0x05 => (Opecode::ORA, Addressing::ZeroPage, 3),
    0x15 => (Opecode::ORA, Addressing::ZeroPageX, 4),
    0x0D => (Opecode::ORA, Addressing::Absolute, 4),
    0x1D => (Opecode::ORA, Addressing::AbsoluteX, 4),
    0x19 => (Opecode::ORA, Addressing::AbsoluteY, 4),
    0x01 => (Opecode::ORA, Addressing::IndirectX, 5),
    0x11 => (Opecode::ORA, Addressing::IndirectY, 6),

    0x49 => (Opecode::EOR, Addressing::Immediate, 2),
    0x45 => (Opecode::EOR, Addressing::ZeroPage, 3),
    0x55 => (Opecode::EOR, Addressing::ZeroPageX, 4),
    0x4D => (Opecode::EOR, Addressing::Absolute, 4),
    0x5D => (Opecode::EOR, Addressing::AbsoluteX, 4),
    0x59 => (Opecode::EOR, Addressing::AbsoluteY, 4),
    0x41 => (Opecode::EOR, Addressing::IndirectX, 5),
    0x51 => (Opecode::EOR, Addressing::IndirectY, 6),

    0x0A => (Opecode::ASL, Addressing::Accumulator, 2),
    0x06 => (Opecode::ASL, Addressing::ZeroPage, 5),
    0x16 => (Opecode::ASL, Addressing::ZeroPageX, 6),
    0x0E => (Opecode::ASL, Addressing::Absolute, 6),
    0x1E => (Opecode::ASL, Addressing::AbsoluteX, 6),

    0x4A => (Opecode::LSR, Addressing::Accumulator, 2),
    0x46 => (Opecode::LSR, Addressing::ZeroPage, 5),
    0x56 => (Opecode::LSR, Addressing::ZeroPageX, 6),
    0x4E => (Opecode::LSR, Addressing::Absolute, 6),
    0x5E => (Opecode::LSR, Addressing::AbsoluteX, 6),

    0x2A => (Opecode::ROL, Addressing::Accumulator, 2),
    0x26 => (Opecode::ROL, Addressing::ZeroPage, 5),
    0x36 => (Opecode::ROL, Addressing::ZeroPageX, 6),
    0x2E => (Opecode::ROL, Addressing::Absolute, 6),
    0x3E => (Opecode::ROL, Addressing::AbsoluteX, 6),

    0x6A => (Opecode::ROR, Addressing::Accumulator, 2),
    0x66 => (Opecode::ROR, Addressing::ZeroPage, 5),
    0x76 => (Opecode::ROR, Addressing::ZeroPageX, 6),
    0x6E => (Opecode::ROR, Addressing::Absolute, 6),
    0x7E => (Opecode::ROR, Addressing::AbsoluteX, 6),

    0x90 => (Opecode::BCC, Addressing::Relative, 2),
    0xB0 => (Opecode::BCS, Addressing::Relative, 2),
    0xF0 => (Opecode::BEQ, Addressing::Relative, 2),
    0xD0 => (Opecode::BNE, Addressing::Relative, 2),
    0x50 => (Opecode::BVC, Addressing::Relative, 2),
    0x70 => (Opecode::BVS, Addressing::Relative, 2),
    0x10 => (Opecode::BPL, Addressing::Relative, 2),
    0x30 => (Opecode::BMI, Addressing::Relative, 2),

    0x24 => (Opecode::BIT, Addressing::ZeroPageX, 3),
    0x2C => (Opecode::BIT, Addressing::Absolute, 4),

    0x4C => (Opecode::JMP, Addressing::Absolute, 3),
    0x6C => (Opecode::JMP, Addressing::IndirectAbsolute, 5),

    0x20 => (Opecode::JSR, Addressing::Absolute, 6),

    0x60 => (Opecode::RTS, Addressing::Implied, 6),

    0x00 => (Opecode::BRK, Addressing::Implied, 7),
    0x40 => (Opecode::RTI, Addressing::Implied, 6),

    0xC9 => (Opecode::CMP, Addressing::Immediate, 2),
    0xC5 => (Opecode::CMP, Addressing::ZeroPage, 3),
    0xD5 => (Opecode::CMP, Addressing::ZeroPageX, 4),
    0xCD => (Opecode::CMP, Addressing::Absolute, 4),
    0xDD => (Opecode::CMP, Addressing::AbsoluteX, 4),
    0xD9 => (Opecode::CMP, Addressing::AbsoluteY, 4),
    0xC1 => (Opecode::CMP, Addressing::IndirectX, 5),
    0xD1 => (Opecode::CMP, Addressing::IndirectY, 6),

    0xE0 => (Opecode::CPX, Addressing::Immediate, 2),
    0xC4 => (Opecode::CPX, Addressing::ZeroPage, 3),
    0xCC => (Opecode::CPX, Addressing::Absolute, 4),

    0xC0 => (Opecode::CPY, Addressing::Immediate, 2),
    0xC4 => (Opecode::CPY, Addressing::ZeroPage, 3),
    0xCC => (Opecode::CPY, Addressing::Absolute, 4),

    0xE6 => (Opecode::INC, Addressing::ZeroPage, 5),
    0xF6 => (Opecode::INC, Addressing::ZeroPageX, 6),
    0xEE => (Opecode::INC, Addressing::Absolute, 6),
    0xFE => (Opecode::INC, Addressing::AbsoluteX, 6),

    0xC6 => (Opecode::DEC, Addressing::ZeroPage, 5),
    0xD6 => (Opecode::DEC, Addressing::ZeroPageX, 6),
    0xCE => (Opecode::DEC, Addressing::Absolute, 6),
    0xDE => (Opecode::DEC, Addressing::AbsoluteX, 6),

    0xE8 => (Opecode::INX, Addressing::Implied, 2),
    0xCA => (Opecode::DEX, Addressing::Implied, 2),
    0xC8 => (Opecode::INY, Addressing::Implied, 2),
    0x88 => (Opecode::DEY, Addressing::Implied, 2),

    0x18 => (Opecode::CLC, Addressing::Implied, 2),
    0x38 => (Opecode::SEC, Addressing::Implied, 2),
    0x58 => (Opecode::CLI, Addressing::Implied, 2),
    0x78 => (Opecode::SEI, Addressing::Implied, 2),
    0xD8 => (Opecode::CLD, Addressing::Implied, 2),
    0xF8 => (Opecode::SED, Addressing::Implied, 2),
    0xB8 => (Opecode::CLV, Addressing::Implied, 2),

    0xA9 => (Opecode::LDA, Addressing::Immediate, 2),
    0xA5 => (Opecode::LDA, Addressing::ZeroPage, 3),
    0xB5 => (Opecode::LDA, Addressing::ZeroPageX, 4),
    0xAD => (Opecode::LDA, Addressing::Absolute, 4),
    0xBD => (Opecode::LDA, Addressing::AbsoluteX,4),
    0xB9 => (Opecode::LDA, Addressing::AbsoluteY, 4),
    0xA1 => (Opecode::LDA, Addressing::IndirectX, 6),
    0xB1 => (Opecode::LDA, Addressing::IndirectY, 5),

    0xA2 => (Opecode::LDX, Addressing::Immediate, 2),
    0xA6 => (Opecode::LDX, Addressing::ZeroPage, 3),
    0xB6 => (Opecode::LDX, Addressing::ZeroPageY, 4),
    0xAE => (Opecode::LDX, Addressing::Absolute, 4),
    0xBE => (Opecode::LDX, Addressing::AbsoluteY, 4),

    0xA0 => (Opecode::LDY, Addressing::Immediate, 3),
    0xA4 => (Opecode::LDY, Addressing::ZeroPage, 3),
    0xB4 => (Opecode::LDY, Addressing::ZeroPageX, 4),
    0xAC => (Opecode::LDY, Addressing::Absolute, 4),
    0xBC => (Opecode::LDY, Addressing::AbsoluteX, 4),

    0x85 => (Opecode::STA, Addressing::ZeroPage, 3),
    0x95 => (Opecode::STA, Addressing::ZeroPageX, 4),
    0x8D => (Opecode::STA, Addressing::Absolute, 4),
    0x9D => (Opecode::STA, Addressing::AbsoluteX, 4),
    0x99 => (Opecode::STA, Addressing::AbsoluteY, 4),
    0x81 => (Opecode::STA, Addressing::IndirectX, 6),
    0x91 => (Opecode::STA, Addressing::IndirectY, 5),

    0x86 => (Opecode::STX, Addressing::ZeroPage, 3),
    0x96 => (Opecode::STX, Addressing::ZeroPageY, 4),
    0x8E => (Opecode::STX, Addressing::Absolute, 4),

    0x84 => (Opecode::STY, Addressing::ZeroPage, 3),
    0x94 => (Opecode::STY, Addressing::ZeroPageX, 4),
    0x8C => (Opecode::STY, Addressing::Absolute, 4),

    0xAA => (Opecode::TAX, Addressing::Implied, 2),
    0x8A => (Opecode::TXA, Addressing::Implied, 2),
    0xA8 => (Opecode::TAY, Addressing::Implied, 2),
    0x98 => (Opecode::TYA, Addressing::Implied, 2),
    0x9A => (Opecode::TXS, Addressing::Implied, 2),
    0xBA => (Opecode::TSX, Addressing::Implied, 2),

    0x48 => (Opecode::PHA, Addressing::Implied, 3),
    0x68 => (Opecode::PLA, Addressing::Implied, 4),
    0x08 => (Opecode::PHP, Addressing::Implied, 3),
    0x28 => (Opecode::PLP, Addressing::Implied, 4),

    0xEA => (Opecode::NOP, Addressing::Implied, 3),
    _ => panic!("panic!"),
  }
}
