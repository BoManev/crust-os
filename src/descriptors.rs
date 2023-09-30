use lazy_static::lazy_static;
use x86_64::structures::{
    gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
    tss::TaskStateSegment,
};
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub fn init() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};

    GDT.0.load();

    unsafe {
        // invalid selectors could break memory
        CS::set_reg(GDT.1.cs);
        load_tss(GDT.1.tss);
    }
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selector) = {
        let mut gdt = GlobalDescriptorTable::new();
        let cs = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selector { cs, tss })
    };
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        // TODO: proper page alloc
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            // Note: no guard page use with caution
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            // stack grows downwards (high -> low)
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

struct Selector {
    cs: SegmentSelector,
    tss: SegmentSelector,
}
