/***************************************************
* boot.S
* Create: 20241229
* Update: 20241230
* Auth: Eduardo Pagotto (edupagotto@gmail.com)
***************************************************/


.global start
.section .boot

start:
# desabilita interrupcoes
csrw mie, zero

# Set sp RAM + 1024
lui sp, %hi(_stack_start)
add sp, sp, %lo(_stack_start)

# should jump into rust_entry
lui t0, %hi(rust_entry)
add t0, t0, %lo(rust_entry)

csrw mepc, t0
mret
