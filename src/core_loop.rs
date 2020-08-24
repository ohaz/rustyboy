extern crate piston_window;

use piston_window::*;

pub fn draw_loop(window_title: &str, gameboy: &mut crate::hardware::gameboy::GameBoy)
{
    let mut window: PistonWindow =
        WindowSettings::new(window_title, [160, 144])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        crate::hardware::cpu::step(gameboy);
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
        });
    }
}