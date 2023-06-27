#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[macro_use]
mod vga_text;

mod ata;
mod io;

use crate::ata::Ata;

const KERNEL_ADDRESS: usize = 0x10000;

#[no_mangle]
pub unsafe extern "C" fn kernel_load(drive: u32) -> ! {
    let mut ata = Ata::new(drive);
    ata.read(7, 80, KERNEL_ADDRESS);

    let kernel_point: fn() = core::mem::transmute(KERNEL_ADDRESS);
    kernel_point();

    loop {}
}   

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

