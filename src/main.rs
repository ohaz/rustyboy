mod core_loop;
mod hardware;

extern crate log;
extern crate simple_logger;

use log::{info};

fn main() {
    simple_logger::init().unwrap();

    let rom = hardware::rom_loader::read_rom("./roms/rom.gbc");
    let rom_name: String = hardware::rom_loader::get_rom_name(&rom);

    let mut gameboy = hardware::gameboy::GameBoy::default();
    gameboy.map_cartridge(&rom);

    info!("ROM Name: {name}", name=rom_name);
    info!("ROM validity: {validity}", validity=hardware::rom_loader::check_valid(&rom));
    core_loop::draw_loop(&rom_name, &mut gameboy);
}