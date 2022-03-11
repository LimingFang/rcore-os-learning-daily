OUTPUT_ARCH(RISCV)
ENTRY(_start)
BASE_ADDESS = 0x80400000;

SECTIONS {
  . = BASE_ADDESS;
  .text : {
    *(.text.start)
    *(.text .text.*)
  }

  . = ALIGN(4K);
  .rodata : {
    *(.rodata .rodata.*)
    *(.srodata .srodata.*)
  }

  . = ALIGN(4K);
  .data : {
    *(.data .data.*)
    *(.sdata .sdata.*)
  }

  . = ALIGN(4K);
  sbss = .;
  .bss : {
    *(.bss .bss.*)
    *(.sbss .sbss.*)
  }
  ebss = .;

  /DISCARD/ : {
    *(.eh_frame)
    *(.debug*)
  }
}