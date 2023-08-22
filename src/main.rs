#![no_std]
#![no_main]

mod vgabuf;

use core::fmt::Write;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vgabuf::WRITER.lock().write_str("Hello World");
    write!(vgabuf::WRITER.lock(), "format: 127.0.0.1:{}", 8000).unwrap();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
