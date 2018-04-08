movi r0, 0
movi r1, 2
movi r2, 2
movi r3, 3
movi r4, -1
cmplt r5, r2, r3
st 0(r0), r5
cmplt r5, r4, r2
st 2(r0), r5
cmplt r5, r1, r2
st 4(r0), r5
cmplt r5, r3, r0
st 6(r0), r5
cmple r5, r2, r3
st 8(r0), r5
cmple r5, r4, r2
st 10(r0), r5
cmple r5, r1, r2
st 12(r0), r5
cmple r5, r3, r0
st 14(r0), r5
cmpeq r5, r2, r3
st 16(r0), r5
cmpeq r5, r4, r2
st 18(r0), r5
cmpeq r5, r1, r2
st 20(r0), r5
cmpeq r5, r3, r0
st 22(r0), r5
cmpltu r5, r2, r3
st 24(r0), r5
cmpltu r5, r4, r2
st 26(r0), r5
cmpltu r5, r1, r2
st 28(r0), r5
cmpltu r5, r3, r0
st 30(r0), r5
cmpleu r5, r2, r3
st 32(r0), r5
cmpleu r5, r4, r2
st 34(r0), r5
cmpleu r5, r1, r2
st 36(r0), r5
cmpleu r5, r3, r0
st 38(r0), r5
movi r0, 0
movi r1, -1
addi r1, r1, -1
st 0(r0), r1
addi r2, r1, -2
st 2(r0), r2
addi r3, r1, 1
st 4(r0), r3
addi r4, r1, 3
st 6(r0), r4
addi r1, r1, 0
st 8(r0), r1
addi r5, r1, 0
st 10(r0), r5
movi r0, 0
movi r1, 0x7d
movhi r1, 0x33
movi r2, 0x68
movhi r2, 0x43
and r3, r1, r2
st 0(r0), r3
or r4, r1, r2
st 2(r0), r4
not r5, r2
st 4(r0), r5
xor r6, r1, r2
st 6(r0), r6
movi r0, 0
movi r1, 1
movi r2, 2
add r3, r1, r2
st 0(r0), r3
movi r2, 0xff
movhi r2, 0x7f
add r3, r2, r1
st 2(r0), r3
movhi r2, 0xff
add r3, r1, r2
st 4(r0), r3
movi r1, 0xff
add r3, r1, r2
st 6(r0), r3
movi r1, 0
movhi r1, 0x80
add r3, r1, r2
st 8(r0), r3
movi r0, 0
movi r1, 1
movi r2, 2
sub r3, r1, r2
st 0(r0), r3
movi r1, 0xff
movi r2, 0xff
sub r3, r1, r2
st 2(r0), r3
movi r2, 0xfe
sub r3, r1, r2
st 4(r0), r3
movi r1, 0
movhi r1, 0x80
movi r2, 1
sub r3, r1, r2
st 6(r0), r3
movi r0, 0xbc
movhi r0, 0xa5
movi r1, 0x22
movhi r1, 0x40
movi r2, 3
movi r3, -2
sha r4, r0, r2
sha r5, r0, r3
shl r6, r1, r2
shl r7, r1, r3
movi r0, 0
st 0(r0), r4
st 2(r0), r5
st 4(r0), r6
st 6(r0), r7
movi r0, 0
movi r1, 1
movi r2, 2
mul r3, r1, r2
mulh r4, r1, r2
mulhu r5, r1, r2
st 0(r0), r3
st 2(r0), r4
st 4(r0), r5
movi r2, 0xff
movhi r2, 0x7f
mul r3, r1, r2
mulh r4, r1, r2
mulhu r5, r1, r2
st 6(r0), r3
st 8(r0), r4
st 10(r0), r5
movhi r2, 0xff
mul r3, r1, r2
mulh r4, r1, r2
mulhu r5, r1, r2
st 12(r0), r3
st 14(r0), r4
st 16(r0), r5
movi r1, 0xff
mul r3, r1, r2
mulh r4, r1, r2
mulhu r5, r1, r2
st 18(r0), r3
st 20(r0), r4
st 22(r0), r5
movi r1, 0
movhi r1, 0x80
mul r3, r1, r2
mulh r4, r1, r2
mulhu r5, r1, r2
st 24(r0), r3
st 26(r0), r4
st 28(r0), r5
movi r0, 0
movi r1, 1
movi r2, 2
div r3, r2, r1
divu r4, r2, r1
st 0(r0), r3
st 2(r0), r4
movi r2, 0xff
movhi r2, 0x7f
div r3, r1, r2
divu r4, r1, r2
st 4(r0), r3
st 6(r0), r4
movhi r2, 0xff
div r3, r1, r2
divu r4, r1, r2
st 8(r0), r3
st 10(r0), r4
movi r1, 0xff
div r3, r1, r2
divu r4, r1, r2
st 12(r0), r3
st 14(r0), r4
movi r1, 0
movhi r1, 0x80
div r3, r1, r2
divu r4, r1, r2
st 16(r0), r3
st 18(r0), r4
movi r0, 64
movi r1, 3
movi r2, 4
movi r3, -2
movi r4, -7
mul r5, r1, r2
st 0(r0), r5
mul r5, r1, r3
st 2(r0), r5
mulh r5, r1, r2
st 4(r0), r5
mulh r5, r1, r3
st 6(r0), r5
mulhu r5, r1, r2
st 8(r0), r5
mulhu r5, r1, r3
st 10(r0), r5
div r5, r2, r1
st 12(r0), r5
div r5, r0, r3
st 14(r0), r5
divu r5, r4, r2
st 16(r0), r5
divu r5, r0, r3
st 18(r0), r5