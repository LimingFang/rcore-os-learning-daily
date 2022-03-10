.altmacro
.align 2;
.globl __restore
.macro RESTORE_GP n
  ld x\n,\n*8(sp)
.endm  
.section .text  
__restore:
  # 此时 a0 是内核栈指针
  mv sp,a0
  # 先恢复 csr
  ld t0,32*8(sp)
  ld t1,33*8(sp)
  csrrw t0,sepc,t0
  csrrw t1,sstatus,t1
  csrrw t2,sscratch,t2
  # 在恢复除了 x2(sp) 外的GP
  ld x1,1*8(sp)
  .set n,3
  .rept 29
    RESTORE_GP %n
    .set n,n+1
  .endr

  # 处理 sscratch 和 sp
  addi sp,sp,34*8
  csrrw sp,sscratch,sp
  sret



