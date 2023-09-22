ENTRY(_kmain)
OUTPUT_FORMAT(binary)

phys = 0x100000;

SECTIONS {
     . = phys;

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