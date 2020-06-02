use super::super::ram;
use super::palette::PaletteList;
use super::super::cassette::roms::Rom;

pub struct Registers {
  ctrl1: u8,
  ctrl2: u8,
  status: u8,
  sprite_addr: u8,
  sprite_data: u16,
  ppu_addr: u16,
  ppu_data: u8,
}

impl Registers {
  pub fn new() -> Registers {
    Registers {
      ctrl1: 0,
      ctrl2: 0,
      status: 0,
      sprite_addr: 0,
      sprite_data: 0,
      ppu_addr: 0,
      ppu_data: 0,
    }
  }

  fn read_status(&mut self) -> u8 {
    let data = self.status;
    self.clear_vblank();
    self.clear_sprite_hit();
    data
  }

  fn clear_vblank(&mut self) {
    self.status &= 0x7F;
  }

  fn clear_sprite_hit(&mut self) {
    self.status &= 0xbF;
  }

  pub fn get_backgound_table_offset(&self) -> u16 {
    if self.ctrl1 & 0x10 == 0x10 {
      0x1000
    } else {
      0x0000
    }
  }

  pub fn get_sprite_table_offset(&self) -> u16 {
    if self.ctrl1 & 0x08 == 0x08 {
      0x1000
    } else {
      0x0000
    }
  }

  pub fn is_background_enable(&self) -> bool {
    self.ctrl2 & 0x08 == 0x08
  }

  pub fn read(&mut self, addr: u16, cassette: &Rom, vram:&mut ram::Ram, palette:&mut PaletteList) -> u8 {
    match addr {
      0x0002 => self.read_status(),
      0x0004 => {println!("oam"); 0},
      0x0007 => self.ppu_data_read(addr, cassette, vram, palette),
      _ => 0,
    }
  }

  fn ppu_data_read(&mut self, addr:u16, cassette: &Rom, vram:&mut ram::Ram, palette:&mut PaletteList) -> u8 {
    let buf = self.ppu_data;
    if addr >= 0x2000 {
      let addr = self.calc_addr(addr);
      if addr >= 0x3F00{
        self.ppu_data = vram.read(addr);
        return palette.read(addr - 0x3f00);
      }
      self.ppu_addr = vram.read(addr) as u16;
    }else{
      self.ppu_addr = cassette.rom_read(addr) as u16;
    }

    buf
  }

  fn calc_addr(&self, addr: u16) -> u16 {
    if addr >= 0x3000 && addr < 0x3f00 {
        addr - 0x3000
    } else {
        addr - 0x2000
    }
}
}
