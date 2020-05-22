use nes_emulator::cassette;

fn main() {
    let cassette = cassette::roms::rom_read("./roms/SHOOT.nes");
    println!("{:?}", cassette);
}
