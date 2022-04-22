# __switch(current_task_ctx_ptr: *mut TaskContext, 
#       next_task_ctx_ptr: *const TaskContext);
  .section .text
  .globl __switch
__switch:
  # a0:current
  # a1:next
  # 1.dump current callee-saved cpu registers to current task ctx.
  # sd src,dst
  sd s0,16(a0)
  sd s1,24(a0)
  sd s2,32(a0)
  sd s3,40(a0)
  sd s4,48(a0)
  sd s5,56(a0)
  sd s6,64(a0)
  sd s7,72(a0)
  sd s8,80(a0)
  sd s9,88(a0)
  sd s10,96(a0)
  sd s11,104(a0)
  # 2.dump sp and ra
  sd sp,8(a0)
  sd ra,0(a0)
  # 3.load next task ctx.
  # ld dst,src
  ld s0,16(a1)
  ld s1,24(a1)
  ld s2,32(a1)
  ld s3,40(a1)
  ld s4,48(a1)
  ld s5,56(a1)
  ld s6,64(a1)
  ld s7,72(a1)
  ld s8,80(a1)
  ld s9,88(a1)
  ld s10,96(a1)
  ld s11,104(a1)
  # 4.load ra and sp
  ld sp,8(a1)
  ld ra,0(a1)

  ret