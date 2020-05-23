struct Status{
  negative: bool,
  overflow: bool,
  reserved: bool,
  dreak_mode: bool,
  decimal_mode: bool,
  interrupt:bool,
  zero: bool,
  carry: bool,
}

#[allow(non_snake_case)]
pub struct Registers{
  A: u8,
  X: u8,
  Y: u8,
  SP: u8,
  PC: u16,
  P: Status,
}

