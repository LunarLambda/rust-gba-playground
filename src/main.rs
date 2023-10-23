#![no_std]
#![no_main]

use gba::video::{DisplayControl, Layers, Mode3};

unsafe fn m3_pixel(x: usize, y: usize, col: u16) {
    (0x0600_0000 as *mut u16).add(x + y * 240).write(col);
}

#[gba::entry]
fn main(gba: gba::Gba) -> ! {
    let mut display = gba.display.mode::<Mode3>();

    display
        .control
        .write(DisplayControl::new().enable_layers(Layers::BG2));

    unsafe {
        m3_pixel(120, 80, 0x001F);
        m3_pixel(136, 80, 0x03E0);
        m3_pixel(120, 96, 0x7C00);
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
