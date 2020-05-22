use std::fs::File;
use std::io::BufReader;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct Cassette {
  programs_rom:Vec<u8>,
  character_ram:Vec<u8>
}

pub fn rom_read(path:&str) -> Cassette {
  let NES_HEADER_SIZE = 0x0010;
  let PROGRAM_ROM_SIZE = 0x4000;
  let CHARACTER_ROM_SIZE = 0x2000;

  let mut file = File::open(path).unwrap();
  let mut buf:Vec<u8> = Vec::new();
  file.read_to_end(&mut buf);

  let programs_rom_pages = buf[4] as usize;
  let character_rom_pages = buf[5] as usize;
  let character_rom_start = NES_HEADER_SIZE + programs_rom_pages * PROGRAM_ROM_SIZE;
  let character_rom_end = character_rom_start + character_rom_pages * CHARACTER_ROM_SIZE;

  let programs_rom = buf[NES_HEADER_SIZE..character_rom_start].to_vec();
  let character_ram = buf[character_rom_start..character_rom_end].to_vec();

  let cassette = Cassette{
    programs_rom:programs_rom,
    character_ram:character_ram
  };

  return cassette;
}