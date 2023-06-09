use core::arch::asm;

#[inline(always)]
pub unsafe fn outb(p: u16, d: u8) {
     asm!(
          "out dx, al",
          in("dx") p,
          in("al") d,
          options(preserves_flags, nomem, nostack)
     );
} 

#[inline(always)]
pub unsafe fn inb(p: u16) -> u8 {
     let r: u8;
     asm!(
          "in al, dx",
          in("dx") p,
          out("al") r,
          options(preserves_flags, nomem, nostack)
     );
     r
} 

#[inline(always)]
pub unsafe fn wait() {
     outb(0x80, 0);
}