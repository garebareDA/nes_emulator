#[derive(Debug, Clone)]
pub struct PpuAddr {
  addr: u16,
  is_lower_addr: bool,
}

impl PpuAddr {
  pub fn new() -> PpuAddr {
    PpuAddr {
      addr: 0,
      is_lower_addr: false,
    }
  }

  pub fn get(&self) -> u16 {
    self.addr
  }

  pub fn update(&mut self, offset: u16) {
    self.addr = offset;
  }

  pub fn write(&mut self, data: u8) {
    if self.is_lower_addr {
      self.addr += data as u16;
      self.is_lower_addr = false;
    } else {
      self.addr = (data as u16) << 8;
      self.is_lower_addr = true;
    }
  }
}
