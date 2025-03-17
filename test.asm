LDR r6 10
CAL main
LOD r6 r3
HLT

main:
    LDR r0 8
    LDR r1 8
    LDR r2 0
    ADD r0 r1 r2
    STR r6 r2
    RET