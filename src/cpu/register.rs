struct Status {
  negative: bool,
  overflow: bool,
  reserved: bool,
  break_mode: bool,
  decimal_mode: bool,
  interrupt: bool,
  zero: bool,
  carry: bool,
}

#[allow(non_snake_case)]
pub struct Registers {
  A: u8,
  X: u8,
  Y: u8,
  SP: u8,
  PC: u16,
  P: Status,
}

impl Registers {
  pub fn new() -> Registers {
    Registers {
      A: 0,
      X: 0,
      Y: 0,
      PC: 0x8000,
      SP: 0xFD,
      P: Status {
        negative: false,
        overflow: false,
        reserved: true,
        break_mode: true,
        decimal_mode: false,
        interrupt: true,
        zero: false,
        carry: false,
      },
    }
  }
}
