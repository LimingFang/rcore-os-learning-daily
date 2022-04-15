# __switch(current_task_ctx_ptr: *mut TaskContext, 
#       next_task_ctx_ptr: *const TaskContext);
  .section .text
  .globl __switch
__switch:
  # a0:current
  # a1:next
  # 1.dump current callee-saved cpu registers to current task ctx.
  # sd src,dst
  sd s0,a0(16)
  sd s1,a0(24)
  sd s2,a0(32)
  sd s3,a0(40)
  sd s4,a0(48)
  sd s5,a0(56)
  sd s6,a0(64)
  sd s7,a0(72)
  sd s8,a0(80)
  sd s9,a0(88)
  sd s10,a0(96)
  sd s11,a0(104)  
  # 2.dump sp and ra
  sd sp,a0(8)
  sd ra,a0(0)
  # 3.load next task ctx.
  # ld dst,src
  ld s0,a1(16)
  ld s1,a1(24)
  ld s2,a1(32)
  ld s3,a1(40)
  ld s4,a1(48)
  ld s5,a1(56)
  ld s6,a1(64)
  ld s7,a1(72)
  ld s8,a1(80)
  ld s9,a1(88)
  ld s10,a1(96)
  ld s11,a1(104)  
  # 4.load ra and sp
  ld sp,a1(8)
  ld ra,a1(0)

  ret