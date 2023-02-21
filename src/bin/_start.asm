.section .text.entry    # declare a new section called .text.entry
.global _start          # mark _start as a global symbol (exported symbol)
_start:                 # start of the _start function
    la sp, bootstacktop # set the stack pointer to point to the top of the boot stack

    call main           # call the main function

    # Shutdown
    li a6, 0
    li a7, 0x53525354
    ecall

.section .bss.stack     # declare a new section called .bss.stack
.align 12               # align the section on a 2^12=4096-byte boundary (page alignment)
.global bootstack       # mark bootstack as a global symbol
bootstack:              # start of the bootstack section
    .space 4096 * 4     # reserve 4096 * 4 = 16,384 bytes of space for the boot stack

.global bootstacktop    # mark bootstacktop as a global symbol
bootstacktop:           # start of the bootstacktop section


# The program starts by declaring a new section called .text.entry, which is where the _start function will be located. The .global directive is used to export the _start symbol so that it can be accessed from other parts of the program.

# Inside the _start function, the la (load address) instruction is used to load the address of bootstacktop into the stack pointer register (sp). This sets up the stack for the program. The call instruction is then used to call the main function.

# Next, a new section called .bss.stack is declared using the .section directive. The .align directive is used to align the section on a 4096-byte boundary (page alignment), which is the default alignment for most RISC-V systems. The .global directive is used to export the bootstack symbol, which marks the start of the boot stack. The .space directive is used to reserve 16,384 bytes of space for the boot stack.

# Finally, the bootstacktop symbol is marked as a global symbol using the .global directive. This marks the end of the boot stack and can be used by other parts of the program to determine the top of the stack.
