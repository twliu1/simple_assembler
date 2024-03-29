movi 0, r0
movi 1, r1
movi 0, r3
mov r1, r2
add r0, r1
stor r0, r3
addi 1, r3
mov r2, r0
cmpi 50, r1
bcond 250, le
load r4, r3
lshi 3, r4
stor r4, r3
movi 0, r0
movi 0, r1
movi 0, r2
movi 0, r3

