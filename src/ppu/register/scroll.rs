#[derive(Debug, Clone)]
enum Enable {
  X,
  Y,
}

#[derive(Debug, Clone)]
pub struct Scroll {
  x: u8,
  y: u8,
  enable: Enable,
}

impl Scroll {
  pub fn new() -> Scroll {
    Scroll {
      x: 0,
      y: 0,
      enable: Enable::X,
    }
  }

  pub fn get_x(&self) -> u8 {
    self.x
  }

  pub fn get_y(&self) -> u8 {
    self.y
  }

  pub fn write(&mut self, data: u8) {
    match self.enable {
      Enable::X => {
        self.enable = Enable::Y;
        self.x = data;
      }
      Enable::Y => {
        self.enable = Enable::X;
        self.y = data;
      }
    }
  }
}
