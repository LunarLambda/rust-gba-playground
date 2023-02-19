#![no_std]
#![no_main]

#[no_mangle]
unsafe extern "C" fn main() -> ! {

    (0x04000000 as *mut u16).write_volatile(0x0403);
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
