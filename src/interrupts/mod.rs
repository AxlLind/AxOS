mod idt;
use idt::InterruptDescriptorTable;

// Pushed on the stack by the CPU before calling the interrupt handler
// For some interrupts an error code is also pushed in the stack.
// References:
// https://os.phil-opp.com/cpu-exceptions/#the-interrupt-stack-frame
// https://wiki.osdev.org/Exceptions
#[derive(Debug,Clone)]
#[repr(C)]
pub struct InterruptStackFrame {
  pub instruction_ptr: u64,
  pub code_segment: u64,
  pub cpu_flags: u64,
  pub stack_ptr: u64,
  pub stack_segment: u64,
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
  static ref IDT: InterruptDescriptorTable = {
    let mut idt = InterruptDescriptorTable::new();
    idt.set_handler(3, breakpoint_handler as u64);
    idt.set_handler(8, double_fault_handler as u64);
    idt
  };
}

pub fn initialize() {
  IDT.load();
}
