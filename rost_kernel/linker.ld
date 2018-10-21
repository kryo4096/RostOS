ENTRY(_start)

SECTIONS
{
  . = 0xFFFF800000000000;
  .text : { *(.text) }
  . = 0xFFFF800001000000;
  .rodata : { *(.rodata) }
  . = ALIGN(0x1000);
  .eh_frame : { *(.eh_frame) }
  . = ALIGN(0x1000);
  .data : { *(.data) }
  . = ALIGN(0x1000);
  .data.rel.ro : { *(.data.rel.ro)}
  . = ALIGN(0x1000);
  .got : { *(.got)}
  . = ALIGN(0x1000);
  .bss : { *(.bss) }
  /DISCARD/ : { *(.comment) *(.debug_loc) *(.debug_str) *(.debug_abbrev) *(.debug_info) *(.debug_macinfo) *(.debug_pubnames) *(.debug_pubtypes)  *(.debug_frame)  *(.debug_line)  *(.debug_ranges) }

}