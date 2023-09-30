use crate::{descriptors, println};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(handle_breakpoint);
        unsafe {
            idt.double_fault
                .set_handler_fn(handle_double_fault)
                .set_stack_index(descriptors::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn handle_breakpoint(stack_frame: InterruptStackFrame) {
    println!("Expection: BREAKPOINT\n\t{:#?}", stack_frame);
}

// x86 can't return from a double fault
extern "x86-interrupt" fn handle_double_fault(stack_frame: InterruptStackFrame, _errno: u64) -> ! {
    panic!("Expection: Double Fault\n\t{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_interrupt() {
    x86_64::instructions::interrupts::int3();
}
