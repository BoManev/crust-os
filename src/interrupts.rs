use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(handle_breakpoint);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn handle_breakpoint(
    stack_frame: InterruptStackFrame
) {
    println!("Expection: BREAKPOINT\n\t{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_interrupt() {
    x86_64::instructions::interrupts::int3();
}
