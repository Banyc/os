.section .text.entry    # declare a new section called .text.entry
.global _start          # mark _start as a global symbol (exported symbol)
_start:                 # start of the _start function
    la sp, boot_stack_top # set the stack pointer to point to the top of the boot stack

    la t0, supervisor_exception_stack_top # load the address of supervisor_exception_stack_top into t0
    csrw sscratch, t0 # set the supervisor exception stack pointer

    call main           # call the main function

    # Shutdown
    li a6, 0
    li a7, 0x53525354
    ecall

.section .bss.stack     # declare a new section called .bss.stack
.align 12               # align the section on a 2^12=4096-byte boundary (page alignment)
.global boot_stack       # mark boot_stack as a global symbol
boot_stack:              # start of the boot_stack section
    .space 4096 * 4     # reserve 4096 * 4 = 16,384 bytes of space for the boot stack

.global boot_stack_top    # mark boot_stack_top as a global symbol
boot_stack_top:           # start of the boot_stack_top section

.section .bss.stack
.align 12
.global supervisor_exception_stack
supervisor_exception_stack:
    .space 4096 * 4

.global supervisor_exception_stack_top
supervisor_exception_stack_top:


# The program starts by declaring a new section called .text.entry, which is where the _start function will be located. The .global directive is used to export the _start symbol so that it can be accessed from other parts of the program.

# Inside the _start function, the la (load address) instruction is used to load the address of boot_stack_top into the stack pointer register (sp). This sets up the stack for the program. The call instruction is then used to call the main function.

# Next, a new section called .bss.stack is declared using the .section directive. The .align directive is used to align the section on a 4096-byte boundary (page alignment), which is the default alignment for most RISC-V systems. The .global directive is used to export the boot_stack symbol, which marks the start of the boot stack. The .space directive is used to reserve 16,384 bytes of space for the boot stack.

# Finally, the boot_stack_top symbol is marked as a global symbol using the .global directive. This marks the end of the boot stack and can be used by other parts of the program to determine the top of the stack.
