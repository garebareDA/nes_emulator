use nes_emulator::cassette;
use nes_emulator::ram;
use nes_emulator::bus;
use nes_emulator::cpu;
use nes_emulator::render;

fn main() {
    let cassette = cassette::roms::Rom::new("./roms/sample1.nes");
    let ram = ram::Ram::new(vec![0; 0x0800]);
    let mut bus = bus::Bus::new(cassette, ram);
    let mut register = cpu::register::Registers::new();
    cpu::process::reset(&mut register, &mut bus);
    loop {
        let cycle = cpu::run::run(&mut register, &mut bus);
        let render = bus.ppu.run((cycle * 3) as usize,  &bus.mmc);
        if render {
            return;
            render::render_backgound(&bus.ppu.background);
            return;
        }
    }
}
