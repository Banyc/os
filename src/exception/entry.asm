.equ REGBYTES, 8

.align 2
exception_entry:
    addi sp, sp, -17*REGBYTES

    sw ra, 0*REGBYTES(sp)
    sw a0, 1*REGBYTES(sp)
    sw a1, 2*REGBYTES(sp)
    sw a2, 3*REGBYTES(sp)
    sw a3, 4*REGBYTES(sp)
    sw a4, 5*REGBYTES(sp)
    sw a5, 6*REGBYTES(sp)
    sw a6, 7*REGBYTES(sp)
    sw a7, 8*REGBYTES(sp)
    sw t0, 9*REGBYTES(sp)
    sw t1, 10*REGBYTES(sp)
    sw t2, 11*REGBYTES(sp)
    sw t3, 12*REGBYTES(sp)
    sw t4, 13*REGBYTES(sp)
    sw t5, 14*REGBYTES(sp)
    sw t6, 15*REGBYTES(sp)

    jal handle_exception

    lw ra, 0*REGBYTES(sp)
    lw a0, 1*REGBYTES(sp)
    lw a1, 2*REGBYTES(sp)
    lw a2, 3*REGBYTES(sp)
    lw a3, 4*REGBYTES(sp)
    lw a4, 5*REGBYTES(sp)
    lw a5, 6*REGBYTES(sp)
    lw a6, 7*REGBYTES(sp)
    lw a7, 8*REGBYTES(sp)
    lw t0, 9*REGBYTES(sp)
    lw t1, 10*REGBYTES(sp)
    lw t2, 11*REGBYTES(sp)
    lw t3, 12*REGBYTES(sp)
    lw t4, 13*REGBYTES(sp)
    lw t5, 14*REGBYTES(sp)
    lw t6, 15*REGBYTES(sp)

    addi sp, sp, 17*REGBYTES

    # Return to `mepc`
    mret
