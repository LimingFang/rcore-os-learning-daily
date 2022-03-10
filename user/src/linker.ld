OUTPUT_ARCH(RISCV)
ENTRY(_start)
BASE_ADDESS = 0x80400000;

SECTIONS {
  . = BASE_ADDESS;
  stext = .;
  .text : {
    *(.text.start)
    *(.text .text.*)
  }

  . = ALIGN(4K);
  etext = .;
  srodata = .;
  .rodata : {
    *(.rodata .rodata.*)
    *(.srodata .srodata.*)
  }

  . = ALIGN(4K);
  erodata = .;
  sdata = .;
  .data : {
    *(.data .data.*)
    *(.sdata .sdata.*)
  }

  . = ALIGN(4K);
  edata = .;
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