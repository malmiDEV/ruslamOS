pub mod idt;
pub mod interrupts;
pub mod isrs_func;

pub use idt::*;
pub use interrupts::*;
pub use isrs_func::*;

use core::arch::asm;

#[inline(always)]
pub unsafe fn halt() {
    asm!("hlt", options(nomem, nostack))
}

#[inline(always)]
pub unsafe fn clear_interrupt() {
    asm!("cli", options(nomem, nostack))
}

#[inline(always)]
pub unsafe fn enable_interrupt() {
    asm!("sti", options(nomem, nostack))
}
