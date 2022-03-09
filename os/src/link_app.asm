  .globl _num_app;
  .section .data;
  .alias 3;
_num_app:
  .quad 3;
  .quad app_0_start;
  .quad app_1_start;
  .quad app_2_start;
  .quad app_2_end;
app_0_start:
  incbin "../../user/src/bin/target/riscv64-unknown-none-elf/release/hello_world.bin"
app_1_end:
  .global app_0_start;
  .global app_0_end;
  .section .data; 

app_1_start:
  incbin "../../user/src/bin/target/riscv64-unknown-none-elf/release/power.bin"
app_1_end:
  .global app_1_start;
  .global app_0_end;
  .section .data; 

app_2_start:
  incbin "../../user/src/bin/target/riscv64-unknown-none-elf/release/store_fault.bin"    
app_2_end:  
  .global app_2_end;
  .global app_2_end;
