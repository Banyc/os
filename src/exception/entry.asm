# - Reference: <https://github.com/rcore-os/rCore-Tutorial/blob/master/os/src/interrupt/interrupt.asm>

.altmacro
.set    REG_SIZE, 8
.set    CONTEXT_SIZE, 34

# Save register to stack
.macro SAVE reg, offset
    sd \reg, \offset * REG_SIZE(sp)
.endm

# Load register from stack
.macro LOAD reg, offset
    ld \reg, \offset * REG_SIZE(sp)
.endm

# Save xn to stack
.macro SAVE_N n
    SAVE x\n, n
.endm

# Load xn from stack
.macro LOAD_N n
    LOAD x\n, n
.endm

    .section .text
    .globl __exception_entry
# Compose a RegisterContext and push it to the stack
__exception_entry:
    # Swap sp from the pre-exception sp to the supervisor sp
    # - Why switching stacks: the thread stack might not be available
    # - ssratch: store the supervisor stack address
    csrrw   sp, sscratch, sp
    # Allocate space for RegisterContext
    addi    sp, sp, -CONTEXT_SIZE * REG_SIZE

    # Save x registers
    SAVE    x1, 1
    # Save the pre-exception sp
    csrr    x1, sscratch
    SAVE    x1, 2
    # Save registers from x3 to x31
    .set    n, 3
    .rept   29
        SAVE_N  %n
        .set    n, n + 1
    .endr

    # Call handle_exception with the following arguments:
    # register_context: &mut RegisterContext
    mv      a0, sp
    jal handle_exception

    .globl __restore
# Exit from exception
__restore:
    # Write exception sp to sscratch
    addi    t0, sp, CONTEXT_SIZE * REG_SIZE
    csrw    sscratch, t0

    # Restore x registers
    LOAD    x1, 1
    # Restore registers from x3 to x31
    .set    n, 3
    .rept   29
        LOAD_N  %n
        .set    n, n + 1
    .endr

    # Restore sp
    # - Why not restore it first: the macro `LOAD` uses `sp` as the base address
    LOAD    x2, 2

    # Jumps back to sepc
    sret
