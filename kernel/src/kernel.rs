#![no_std]
#![no_main]
#![allow(unused_imports)]

use core::arch::global_asm;
use core::panic::PanicInfo;
use rtos::uart;

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

    let u0 = uart::Uart::from_base_address(uart::UART0_BASE_ADDR);
    u0.init(115200);
    loop {
        u0.send_byte_blocking(87 as u8); // W
        u0.send_byte_blocking(101 as u8); // e
        u0.send_byte_blocking(108 as u8); // l
        u0.send_byte_blocking(99 as u8); // c
        u0.send_byte_blocking(111 as u8); // o
        u0.send_byte_blocking(109 as u8); // m
        u0.send_byte_blocking(101 as u8); // e
        u0.send_byte_blocking(10 as u8); // line feed
        u0.send_byte_blocking(13 as u8); // carrie return
    }
}
