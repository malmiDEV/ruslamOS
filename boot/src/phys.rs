pub fn alloc(sz: usize, align: usize, addr: &mut u32) -> u32 {
  let mut free_addr: u32 = 0x100000;

  let mut size = sz as u32;
  let mut align = align as u32;
  
  if align == 1 && (free_addr & 0xFFFFF000) != 0 {
    free_addr &= 0xFFFFF000;
    free_addr += 0x1000
  }

  if *addr == 0 {
    *addr = free_addr
  }
  
  let return_addr = free_addr;
  free_addr += size;
  
  return_addr
}