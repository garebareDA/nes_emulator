pub struct PaletteList {
  palette: Vec<u8>
}

impl PaletteList {
  pub fn new() -> PaletteList {
    PaletteList{
      palette:vec!(0; 0x20)
    }
  }

  pub fn pallet_get(self, palette_id: u8) ->  Vec<u8>{
    let offset = 0x00;
    let start = (palette_id * 4 + offset) as usize;
    let end = start + 4;
    (start..end).map(|p| self.palette[0x00]).collect()
  }
}
