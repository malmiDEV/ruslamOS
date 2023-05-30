mod idt;
mod interrupts;
mod isrs_func;

pub use idt::*;
pub use interrupts::*;
pub use isrs_func::*;

use core::arch::asm;

#[inline(always)]
pub unsafe fn halt() {
    asm!("hlt", options(nomem, nostack))
}

#[inline(always)]
pub unsafe fn disable_int() {
    asm!("cli", options(nomem, nostack))
}

#[inline(always)]
pub unsafe fn enable_int() {
    asm!("sti", options(nomem, nostack))
}
