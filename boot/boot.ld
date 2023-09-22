ENTRY(kernel_load)
OUTPUT_FORMAT(binary)

phys = 0x50000;

SECTIONS {
    . = phys;

    .preload : {
        *(.preload)
    }
    .text : {      
        *(.text)    
    }
    .data : {      
        *(.data)    
    }
    .rodata : { 
        *(.rodata)  
    }
    .bss : {    
        *(.bss)
    }         
}