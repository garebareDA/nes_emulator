use super::super::cassette::roms;
use super::super::mmc;
use super::super::ram::Ram;
use super::palette;
use super::tile;

pub struct Background {
  pub tiles: Vec<tile::Tile>,
}

impl Background {
  pub fn new() -> Background {
    Background { tiles: Vec::new() }
  }

  pub fn crear(&mut self) {
    self.tiles = Vec::new();
  }

  pub fn build_line(
    &mut self,
    vram: &Ram,
    crom: &roms::Rom,
    tile: tile::SpritePosition,
    config: &mut tile::SpriteConfig,
    palette: &palette::PaletteList,
    mmc: &mmc::Mmc,
  ) {
    let clamped_tile_y = tile.1 % 30;
    let table_id_offset = if (tile.1 / 30) % 2 == 0 { 0 } else { 2 };
    for x in 0..(32 + 1) {
      let tile_x = x + tile.0;
      let clamped_tile_x = tile_x % 32;
      let name_table_id = ((tile_x / 32) % 2) + table_id_offset;
      config.offset_addr_by_name_table = Some((name_table_id as u16) * 0x400);
      let position: tile::SpritePosition = (clamped_tile_x as u8, clamped_tile_y as u8);
      let tiles = tile::Tile::new(vram, crom, &position, config, palette, mmc);
      self.tiles.push(tiles);
    }
  }
}
