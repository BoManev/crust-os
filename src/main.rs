#![no_std]
#![no_main]

mod vgabuf;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("Hello World{}", "!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
