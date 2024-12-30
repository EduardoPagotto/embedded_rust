#![no_std]
#![no_main]
#![allow(unused_imports)]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("boot.S"));
// mesmo que a linha de cima mas com escobo local
//static RISCV_BOOT_ASM: &str = include_str!("boot.S");

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn rust_entry() {
    //println!("Hello, world!");
}
