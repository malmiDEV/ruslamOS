use core::arch::asm;

const PAGE_PER_TABLE: usize = 1024;
const TABLE_PER_DIR: usize = 1024;
const PAGE_SIZE: usize = 4096;

#[repr(align(4096))]
pub struct PageDirectory {
    page_directory: [u32; TABLE_PER_DIR]
}

impl PageDirectory {
    pub fn new() -> Self {
        Self {
            page_directory: [0u32; TABLE_PER_DIR]
        }
    }
}

#[repr(align(4096))]
pub struct PageTable {
    page_table: [u32; PAGE_PER_TABLE]
}

impl PageTable {
    pub fn new() -> Self {
        Self {
            page_table: [0u32; PAGE_PER_TABLE]
        }
    }
}

pub struct VirtualMem {
    page_dir: PageDirectory,
    page_tb: PageTable
}

impl VirtualMem {
    pub fn new() -> Self {
        let mut page_dir = PageDirectory::new();
        for i in 0..1024 {
            page_dir.page_directory[i] = 0x2 as u32;
        }

        let mut page_tb = PageTable::new();
        for i in 0..1024 {
            page_tb.page_table[i] = ((i * 0x1000) | 3) as u32;
        }

        page_dir.page_directory[0] = ((page_tb.page_table.as_ptr() as usize) | 3) as u32;

        Self {
            page_dir,
            page_tb
        }
    }
    
    pub unsafe fn init_virtual_memory(&mut self) {
        let page_dir: *const u32 = self.page_dir.page_directory.as_ptr();

        // println!("{:#X}", *self.page_dir.page_directory.as_ptr());
        asm!("mov cr3, {dir}", dir = in(reg) page_dir);
        asm!("mov eax, cr0; or eax, 0x80000001; mov cr0, eax");
    }
}
