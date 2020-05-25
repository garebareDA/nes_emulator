use super::cassette;
use super::ram;

pub struct Bus {
  program: cassette::roms::Rom,
  work_ram: ram::Ram,
}

impl Bus {
  pub fn new(program: cassette::roms::Rom, work_ram: ram::Ram) -> Bus {
    return Bus {
      program: program,
      work_ram: work_ram,
    };
  }

  pub fn read_word(&mut self, addr: u16) -> u16 {
    let lower = self.read(addr) as u16;
    let upper = self.read(addr + 1) as u16;
    (upper << 8 | lower) as u16
  }

  pub fn read(&mut self, addr: u16) -> u8 {
    match addr {
      0x0000..=0x1FFF => self.work_ram.read(addr & 0x07FF),
      0x6000..=0x7FFF => {
        println!(
          "Not implemented. This area is battery backup ram area 0x{:x}",
          addr
        );
        0
      }
      0x8000..=0xBFFF => self.program.rom_read(addr - 0x8000),
      0xC000..=0xFFFF if self.program.rom_size() <= 0x4000 => self.program.rom_read(addr - 0xC000),
      0xC000..=0xFFFF => self.program.rom_read(addr - 0x8000),
      _ => panic!("panic!"),
    }
  }

  pub fn write(&mut self, addr: u16, data: u8) {
    match addr {
      0x0000..=0x1FFF => self.work_ram.write(addr & 0x07FF, data),
      0x6000..=0x7FFF => {
        println!(
          "Not implemented. This area is battery backup ram area 0x{:x}",
          addr
        );
      }
      _ => panic!("panic"),
    }
  }
}
