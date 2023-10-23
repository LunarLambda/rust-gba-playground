#![no_std]
#![no_main]

#[gba::entry]
unsafe fn main(_: gba::Gba) -> ! {

    (0x04000000 as *mut u16).write_volatile(0x0403);
    (0x06000000 as *mut u16).add(240 * 80 + 120).write(0x001F);
    (0x06000000 as *mut u16).add(240 * 80 + 136).write(0x03E0);
    (0x06000000 as *mut u16).add(240 * 96 + 120).write(0x7C00);
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
