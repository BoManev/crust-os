#![no_std]
#![no_main]

mod vgabuf;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("Hello World{}", "!");
    panic!("Panicing...");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
