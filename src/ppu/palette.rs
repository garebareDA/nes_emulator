#[derive(Debug, Clone)]
pub struct PaletteList {
  palette: Vec<u8>,
}

impl PaletteList {
  pub fn new() -> PaletteList {
    PaletteList {
      palette: vec![0; 0x20],
    }
  }

  pub fn read(&self, addr: u16) -> u8 {
    if self.clone().is_sprite_mirror(addr) {
      return self.palette[(addr - 0x10) as usize];
    }

    if self.clone().is_background_mirror(addr) {
      return self.palette[0x00];
    }

    self.palette[addr as usize]
  }

  fn is_background_mirror(self, addr: u16) -> bool {
    (addr == 0x04) || (addr == 0x08) || (addr == 0x0c)
  }

  fn is_sprite_mirror(self, addr: u16) -> bool {
    (addr == 0x10) || (addr == 0x14) || (addr == 0x18) || (addr == 0x1c)
  }

  pub fn pallet_get(self, palette_id: u8) -> Vec<u8> {
    let offset = 0x00;
    let start = (palette_id * 4 + offset) as usize;
    let end = start + 4;
    (start..end).map(|p| self.palette[0x00]).collect()
  }
}
