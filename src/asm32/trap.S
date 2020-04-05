# trap.S
# In the future our trap vector will go here.

.global asm_trap_vector
# This will be our trap vector when we start
# handling interrupts.
asm_trap_vector:
    .cfi_startproc
    csrr t0, mcause
    csrr t1, mepc
    csrr t2, mtval
    j asm_trap_vector
    .cfi_endproc
