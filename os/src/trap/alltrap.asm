  .globl __alltraps
  .globl __restore
  .align 2
  .macro SAVE_GP n
    sd x\n, \n*8(sp)
  .endm
__alltraps:
  # 从 sscratch 中获取 KERNEL_STACK sp
  csrrw sp,scratch,sp
  addi sp,sp,-34 * 8
  # 此时sp存内核栈指针，scratch存用户栈指针
  sd x1, 1*8(sp)
  .set n 3
  .rept 27
    SAVE_GP %n
    .set n,n+1
  .endr 
  #此时通用寄存器还剩x2(sp)
  csrr t0, sstatus
  csrr t1, sepc
  sd t0 32*8(sp)
  sd t1 33*8(sp)
  csrr t2, scratch
  sd t2,2*8(sp)
  # 此时通用寄存器和两个csr保存完毕
  # 准备将控制权给 rust,trap_handler第一个
  # 参数是 TrapCtx 的地址，即当前的sp
  mv a0,sp
  call trap_handler
  

  
