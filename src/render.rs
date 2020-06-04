use super::ppu::background::Background;
pub fn render_backgound(background: &Background){
  for bg in &background.tiles {
    for i in 0..8 {
      for j in 0..8 {
        print!("{:?}", bg.sprite[i][j]);
      }
    }
    print!("\n");
  }
}