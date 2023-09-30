#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use crusty_os::{exit_qemu, serial_print};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow...\t");
    crusty_os::descriptors::init();
    init_test_idt();

    stack_overflow();

    panic!("Failed to handle stackoverflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    // prevent tail recursion optimizations
    volatile::Volatile::new(0).read();
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_handle_double_fault)
                .set_stack_index(crusty_os::descriptors::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

extern "x86-interrupt" fn test_handle_double_fault(
    _stack_frame: InterruptStackFrame,
    _errno: u64,
) -> ! {
    serial_print!("[ok]");
    exit_qemu(crusty_os::QemuExitCode::Success);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crusty_os::test_panic_handler(info)
}
