use core::arch::asm;

#[inline(always)]
pub fn outb(p: u16, d: u8) {
     unsafe { 
          asm!(
               "out dx, al",
               in("dx") p,
               in("al") d,
          ) 
     }
} 

#[inline(always)]
pub fn inb(p: u16) -> u8 {
     let r: u8 = 0;
     unsafe { 
          asm!(
               "in al, dx",
               in("al") r,
               in("dx") p,
          ) 
     }
     r
} 

#[inline(always)]
pub fn nop() {
     unsafe {
          asm!("nop");
     }
}