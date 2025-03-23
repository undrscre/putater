; initialize device
PGE 250
LDR r0 1
STR r0 0

; render loop
PGE 251
LDR r14 255
CAL gradient
CAL loop
HLT

loop:
    STR r0 7          ; render framebuffer
    CAL loop
    
gradient:
gradient_loop:
    LDR r15 255       
    CAL line          
    SUB r14 r0 r14    
    BRH NE gradient_loop
    RET              

line:
line_loop:
    STR r15 0         ; write r
    STR r14 1         ; write g
    STR r15 3         ; write x
    STR r14 4         ; write y
    STR r0 5          ; write pixel
    SUB r15 r0 r15    ; decrement r15
    BRH NE line_loop  ; loop until 0
    RET               