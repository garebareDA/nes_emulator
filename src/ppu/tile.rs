use super::super::cassette::roms;
use super::super::mmc;
use super::super::ram::Ram;
use super::palette;

pub type SpritePosition = (u8, u8);

pub struct SpriteConfig {
  pub offset_addr_by_name_table: Option<u16>,
  pub offset_addr_by_background_table: u16,
  pub offset_addr_by_sprite_table: u16,
  pub is_horizontal_mirror: bool,
  pub is_background_enable: bool,
}

pub struct Tile {
  sprite: Vec<Vec<u8>>,
  plallet: Vec<u8>,
}

impl Tile {
  pub fn new(
    vram: &Ram,
    crom: &roms::Rom,
    position: &SpritePosition,
    config: &SpriteConfig,
    plallet: &palette::PaletteList,
    mmc: &mmc::Mmc,
  ) -> Tile {
    let block_id = ((position.0 % 4) / 2) + (((position.1 % 4) / 2) * 2);
    let splite_id = get_sprite_id(vram, position, config);
    let attr = get_attribute(vram, position, config);
    let pallet_id = (attr >> (block_id * 2)) & 0x03;
    let sprite = build(crom, splite_id, config.offset_addr_by_background_table, &mmc);
    let get = plallet.clone().pallet_get(pallet_id);
    Tile{
      sprite: sprite,
      plallet: get,
    }
  }
}

pub fn get_sprite_id(vram: &Ram, position: &SpritePosition, config: &SpriteConfig) -> u8 {
  let tile_number = position.1 as u16 * 32 + position.0 as u16;
  let addr = tile_number + config.offset_addr_by_name_table.unwrap();
  let data = vram.read(addr);
  data
}

pub fn get_attribute(vram: &Ram, position: &SpritePosition, config: &SpriteConfig) -> u8 {
  let addr = 0x03c0
    + ((position.0 / 4) + (position.1 / 4 * 8)) as u16
    + config.offset_addr_by_name_table.unwrap();

  if !config.is_horizontal_mirror {
    return vram.read(addr);
  }

  if (addr >= 0x0400 && addr < 0x0800) || addr >= 0x0C00 {
    return vram.read(addr - 0x400 as u16);
  }

  return vram.read(addr);
}

pub fn build(cram: &roms::Rom, sprite_id: u8, offset: u16, mmc: &mmc::Mmc) -> Vec<Vec<u8>> {
  let mut sprite: Vec<Vec<u8>> = (0..8).into_iter().map(|_| vec![0; 8]).collect();
    for i in 0..16 {
      for j in 0..8 {
        let addr = ((sprite_id + (1 as u8)) as u16) * 16 + i + offset;
        let ram = cram.rom_read(mmc.create_chram_addr(addr));
        if ram & (0x80 >> j) as u8 != 0 {
          sprite[((1 as u16) * 8 + i % 8) as usize][j] += (0x01 << (i / 8)) as u8;
        }
      }
    }
  sprite
}

