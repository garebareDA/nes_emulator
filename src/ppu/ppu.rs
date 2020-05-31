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

  
}