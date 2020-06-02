use super::super::cassette::roms::Rom;
use super::super::mmc::Mmc;
use super::super::ram;
use super::background;
use super::palette;
use super::register;
use super::tile;

#[derive(Debug, Clone)]
pub struct PPU {
  pub cycle: usize,
  pub line: usize,
  register: register::Registers,
  background: background::Background,
  cassette: Rom,
  vram: ram::Ram,
  palette: palette::PaletteList,
  pub config: bool,
}

impl PPU {
  pub fn new(cassette: Rom, config: bool) -> PPU {
    PPU {
      cycle: 0,
      line: 0,
      register: register::Registers::new(),
      background: background::Background::new(),
      cassette: cassette,
      vram: ram::Ram::new(vec![0; 0x0200]),
      palette: palette::PaletteList::new(),
      config: config,
    }
  }

  pub fn run(&mut self, cycle: usize, mmc: &Mmc) -> bool {
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

    let scroll_x = self.register.scroll.get_x();
    let scroll_y = self.register.scroll.get_y();

    if self.line <= 240 && self.line % 8 == 0 {
      let mut config = tile::SpriteConfig {
        offset_addr_by_name_table: None,
        offset_addr_by_background_table: self.register.get_background_table_offset(),
        offset_addr_by_sprite_table: self.register.get_sprite_table_offset(),
        is_horizontal_mirror: self.config,
        is_background_enable: self.register.is_background_enable(),
      };

      let tile_x =
        ((scroll_x as usize + (self.register.get_name_table_id() % 2) as usize * 256) / 8) as u8;
      let tile_y = self.register.scroll.get_y();

      self.background.build_line(
        &self.vram,
        &self.cassette,
        (tile_x, tile_y),
        &mut config,
        &self.palette,
        mmc,
      );

      if self.line >= 262 {
        return true;
      }
    }
    return false;
  }

  pub fn write(&mut self, addr: u16, data: u8) {
    self.register.write(addr, data, &self.cassette, &mut self.vram, &mut self.palette);
  }
}
