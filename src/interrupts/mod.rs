mod idt;
use idt::InterruptDescriptorTable;
mod gdt;
use gdt::{GlobalDescriptorTable, TaskSegmentSelector};

// Used to load the IDT and GDT tables
#[repr(packed)]
#[allow(unused)]
struct DescriptorTablePtr {
  size: u16,
  base_ptr: u64,
}

// Pushed on the stack by the CPU before calling the interrupt handler
// For some interrupts an error code is also pushed in the stack.
// References:
// https://os.phil-opp.com/cpu-exceptions/#the-interrupt-stack-frame
// https://wiki.osdev.org/Exceptions
#[derive(Debug,Clone)]
#[repr(C)]
struct InterruptStackFrame {
  instruction_ptr: u64,
  code_segment: u64,
  cpu_flags: u64,
  stack_ptr: u64,
  stack_segment: u64,
}

extern "x86-interrupt" fn breakpoint_handler(frame: &mut InterruptStackFrame) {
  dbg!("breakpoint interrupt!");
  dbg!("{:x?}", frame);
  loop {}
}

extern "x86-interrupt" fn double_fault_handler(frame: &mut InterruptStackFrame, _err_code: u64) -> ! {
  dbg!("double fault interrupt!");
  dbg!("{:x?}", frame);
  loop {}
}

lazy_static! {
  static ref TSS: TaskSegmentSelector = {
    let mut tss = TaskSegmentSelector::new();
    // TODO: Allocate this memory instead
    static mut INTERRUPT_STACK: [u8; 4096] = [0; 4096];
    tss.set_interrupt_stack(1, unsafe { &INTERRUPT_STACK });
    tss
  };
}

lazy_static! {
  static ref GDT: GlobalDescriptorTable = {
    let mut gdt = GlobalDescriptorTable::new();
    let tss_segment = gdt::tss_segment(&TSS);
    gdt.push(gdt::null_segment());
    gdt.push(gdt::kernel_code_segment());
    gdt.push(tss_segment.0);
    gdt.push(tss_segment.1);
    gdt
  };
}

lazy_static! {
  static ref IDT: InterruptDescriptorTable = {
    let mut idt = InterruptDescriptorTable::new();
    idt[3].set_handler(breakpoint_handler as u64);
    idt[8].set_handler(double_fault_handler as u64).with_ist(1);
    idt
  };
}

pub fn initialize() {
  GDT.load();
  // safe, we know these are valid indexes into the GDT
  unsafe { gdt::set_cs(8); }
  unsafe { gdt::load_tss(16); }
  IDT.load();
}
