use super::register;
use super::background;
use super::super::cassette::roms::Rom;
use super::super::ram;

pub struct PPU {
  pub cycle: usize,
  pub line: usize,
  register: register::Registers,
  background: background::Background,
  cassette: Rom,
  vram: ram::Ram,
}

impl PPU {
  pub fn new (cassette: Rom) -> PPU {
    PPU {
      cycle: 0,
      line: 0,
      register: register::Registers::new(),
      background: background::Background::new(),
      cassette: cassette,
      vram: ram::Ram::new(vec![0; 0x0200]),
    }
  }

  pub fn run(&mut self, cycle: usize) -> bool {
    let cycle = self.cycle + cycle;
    if cycle < 341 {
      self.cycle = cycle;
      return false;
    }

    if self.line == 0 {
      self.background.crear();
    }

    self.cycle = cycle - 134;
    self.line += 1;

    if self.line <= 240 && self.line % 8 == 0 {
      
    }
    return true;
  }
}