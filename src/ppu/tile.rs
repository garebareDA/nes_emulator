use super::super::ram::Ram;
use super::super::cassette::roms;
use super::super::mmc;

pub type SpritePosition = (u8, u8);

pub struct SpriteConfig {
  pub offset_addr_by_name_table: Option<u16>,
  pub offset_addr_by_background_table: u16,
  pub offset_addr_by_sprite_table: u16,
  pub is_horizontal_mirror: bool,
  pub is_background_enable: bool,
}

struct Tile {
  sprite : Vec<Vec<u8>>,
  plallet : Vec<u8>,
}

impl Tile {
  pub fn new(
    vram: &Ram,
    crom: &roms::Rom,
    position: &SpritePosition,
    config: &SpriteConfig,
    plallet: Vec<u8>,
    mmc: &mmc::Mmc,
  ){
    let block_id = ((position.0 % 4) / 2) + (((position.1 % 4) / 2) * 2);
    let splite_id = get_sprite_id(vram, position, config);
    
  }
}

pub fn get_sprite_id(vram: &Ram, position: &SpritePosition, config: &SpriteConfig) -> u8 {
  let tile_number = position.1 as u16 * 32 + position.0 as u16;
  let addr = tile_number + config.offset_addr_by_name_table.unwrap();
  let data = vram.read(addr);
  data
}