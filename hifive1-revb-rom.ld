/****************************************************************************
 *
 *   Copyright (C) 2023 bsvtgc@gmail.com. All rights reserved.
 *   Author: Vincent <bsvtgc@gmail.com>
 *
 ****************************************************************************/


/* Comments here */

OUTPUT_ARCH("riscv")

/* Entry point */

ENTRY(start) # alterado

/* Memory Layout */

/*Flash memory on E31 range 0x2000_0000 - 0x3FFF_FFFF, LENGHT = 0x20000000
- 512MB max support for off-chip flash */

MEMORY
{
    ram        : ORIGIN = 0x80000000, LENGTH = 0x4000
    rom        : ORIGIN = 0x20000000, LENGTH = 0x1000000     /* 0x1000000 16MB off-chip */
}

/* SECTIONS Commmand
SECTIONS { ...
  secname : {
    contents
  }
... }
In above, secname is the output section name, content is the
content that goes into the section. The output section
would be created only if there is any content.

SECTIONS
{
  . = 0x10000;
  .text : { *(.text) }
  . = 0x8000000;
  .data : { *(.data) }
  .bss : { *(.bss) }
  _etext = .;
}
You write the `SECTIONS' command as the keyword `SECTIONS',
followed by a series of symbol assignments and output section descriptions enclosed in curly braces.

The symbol `_etext' will be defined as the address following the last `.bss' input section
*/

SECTIONS
{
    .text :  /* GAS dont accept .code section */
    {
        KEEP(*(.boot))
        KEEP(*(.entry))
        KEEP(*(.mtvec_table))
        *(.text*)
    }
    > rom
    . = ALIGN(4);

    .data :
    {
        *(.data*)
    }
    > rom
    . = ALIGN(4);

  . = 0x80002000; /* Its set at 0x80002000 ( instead of 0x80000000) to use it for RUST */

    /* Set Stack in RAM */
    PROVIDE(_stack_start = .);
}
