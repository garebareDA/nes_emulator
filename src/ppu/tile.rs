use super::ram::Ram;
use super::cassette::roms;

pub type SpritePosition = (u8, u8);

pub struct SpriteConfig {
  pub offset_addr_by_name_table: Option<u16>,
  pub offset_addr_by_background_table: u16,
  pub offset_addr_by_sprite_table: u16,
  pub is_horizontal_mirror: bool,
  pub is_background_enable: bool,
}

struct Tile {
  sprite : <Vec<Vec<u8>>>,
  plallet : Vec<u8>,
}

impl Tile {
  pub fn new(
    vram: &Ram
    croms &roms::Rom
    position: &SpritePosition,
    config: &SpriteConfig,
    plallet: Vec<u8>,
    
  )
}