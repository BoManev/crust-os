#![no_std]
#![no_main]

mod vgabuf;

use core::panic::PanicInfo;
use vgabuf::print_s;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_s();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
