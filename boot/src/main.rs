#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[macro_use]
mod vga_text;

mod ata;
mod io;

use crate::ata::Ata;

const KERNEL_ADDRESS: usize = 0x100000;

#[no_mangle]
pub unsafe extern "C" fn kernel_load(drive: u32) -> ! {
    println!("RuslamOS Boot Loader");
    println!("Kernel Load at {:#X}", KERNEL_ADDRESS);    
    println!("DISK: {:#X}", drive);
    
    println!("\nReading..");
    let mut ata = Ata::new(drive);
    ata.read(0, 1, KERNEL_ADDRESS);

    let kernel_point: fn() = core::mem::transmute(KERNEL_ADDRESS);
    // kernel_point();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

