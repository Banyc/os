OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;
    start = .;

    .text : {
        stext = .;
        *(.text.entry)
        *(.text .text.*)
        . = ALIGN(4K);
        etext = .;
    }

    .rodata : {
        srodata = .;
        *(.rodata .rodata.*)
        . = ALIGN(4K);
        erodata = .;
    }

    .data : {
        sdata = .;
        *(.data .data.*)
        . = ALIGN(4K);
        edata = .;
    }

    .bss : {
        sbss = .;
        *(.bss .bss.*)
        . = ALIGN(4K);
        ebss = .;
    }

    end = .;
}
