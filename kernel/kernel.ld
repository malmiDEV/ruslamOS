ENTRY(_kmain)
OUTPUT_FORMAT(binary)

phys = 0x00010000;

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