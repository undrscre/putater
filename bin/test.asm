define TEST 255
LDR r15 TEST
LDR r3 2
CAL main
HLT

main: 
    ADD r3 r1 r2
    ADD r3 r2 r1
    CAL main
