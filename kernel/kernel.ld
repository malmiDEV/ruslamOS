ENTRY(_kmain)
OUTPUT_FORMAT(binary)

phys = 0x00100000;

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