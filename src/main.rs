use nes_emulator::cassette;

fn main() {
    let cassette = cassette::roms::Rom::new("./roms/SHOOT.nes");
    println!("{:?}", cassette);
}
