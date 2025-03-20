; initialize device
PGE 250
LDR r0 1
LDR r1 0
STR r0 r1

; render loop
PGE 251
CAL loop
HLT

loop:

    ; pixel(x)
    LDR r5 3
    LDR r3 50
    STR r3 r5

    ; pixel(y)
    LDR r5 4
    STR r3 r5

    ; pixel(r)
    LDR r2 0
    LDR r3 255
    STR r3 r2

    LDR r3 5
    LOD r3 r15
    LDR r3 7
    LOD r3 r15

    CAL loop