ENTRY(_kmain)
OUTPUT_FORMAT(binary)

phys = 0x00100000;

SECTIONS {
     . = phys;

     .text : { 
          _text = .;       
          *(.text)    
     }
     .data : { 
          _data = .;       
          *(.data)    
     }
     .rodata : { 
          _rodata = .;     
          *(.rodata)  
     }
     .bss : {
          _bss = .;        
          *(.bss)
     }         

     _end = .;
}