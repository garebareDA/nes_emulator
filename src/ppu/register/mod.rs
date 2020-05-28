pub mod scroll;

pub struct Registers {
  ctrl1 : u8,
  ctrl2 : u8,
  status: u8,
  sprite_addr : u8,
  sprite_data : u16,
  scroll: scroll::Scroll,
  ppu_addr : u16,
  ppu_data: u8,
}

impl Registers {
  pub fn new() -> Registers {
    Registers{
      ctrl1: 0,
      ctrl2: 0,
      status: 0,
      sprite_addr: 0,
      sprite_data: 0,
      scroll: scroll::Scroll::new(),
      ppu_addr: 0,
      ppu_data: 0,
    }
  }
}