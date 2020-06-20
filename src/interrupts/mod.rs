mod idt;
use idt::InterruptDescriptorTable;
mod gdt;
use gdt::GlobalDescriptorTable;

// Used to load both the IDT and GDT tables
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

extern "x86-interrupt" fn double_fault_handler(frame: &mut InterruptStackFrame, err_code: u64) -> ! {
  dbg!("double fault interrupt!");
  dbg!("{:x?}", frame);
  dbg!("err code {}", err_code);
  loop {}
}

lazy_static! {
  static ref GDT: GlobalDescriptorTable = {
    let mut gdt = GlobalDescriptorTable::new();
    gdt.push(gdt::null_segment());
    gdt.push(gdt::kernel_code_segment());
    gdt
  };
}

lazy_static! {
  static ref IDT: InterruptDescriptorTable = {
    let mut idt = InterruptDescriptorTable::new();
    idt.set_handler(3, breakpoint_handler as u64);
    idt.set_handler(8, double_fault_handler as u64);
    idt
  };
}

pub fn initialize() {
  GDT.load();
  // safe since we know 1 is a valid code segment index
  unsafe { gdt::set_cs(1); }
  IDT.load();
}
