#![feature(asm_const)]
#![no_std]
#![no_main]
use core::panic::PanicInfo;

const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;

#[inline]
fn get_abi_addr() -> usize {
    let abi_tb_addr: usize;
    unsafe {
        core::arch::asm!(
            "mv {}, a7", 
            out(reg) abi_tb_addr,
        )
    };
    abi_tb_addr
}

fn putchar(abi_tb_addr: usize, c: u8) {
    // do not use variable to directly replace register t2
    unsafe {
        core::arch::asm!("
        li     t0, {abi_num}
        slli   t0, t0, 3
        add     t1, t0, t2
        ld     t1, (t1)
        jalr   t1",
        abi_num = const SYS_PUTCHAR,
        in("t2") abi_tb_addr,
        in("a0") c,
        )
    }
}

// must add no_mangle
#[no_mangle]
fn puts(abi_tb_addr: usize, s: &str) {
    for c in s.bytes() {
        putchar(abi_tb_addr, c);
    }
}

fn shutdown(abi_tb_addr:usize) -> ! {
    unsafe {
        core::arch::asm!("
        li     t0, {abi_num}
        slli   t0, t0, 3
        add     t1, {addr}, t0
        ld     t1, (t1)
        jalr   t1",
        abi_num = const SYS_TERMINATE,
        addr = in(reg) abi_tb_addr,
        options(noreturn),
        )
    }
}

#[no_mangle]
unsafe extern "C" fn _start() {
    let arg0: u8 = b'C';
    // need to save register ra
    core::arch::asm!("
        addi   sp, sp, -16
        sd     ra, 8(sp)
        li     t0, {abi_num}
        slli   t0, t0, 3
        add     t1, a7, t0
        ld     t1, (t1)
        jalr   t1
        ld     ra, 8(sp)
        addi   sp, sp, 16",
        abi_num = const SYS_PUTCHAR,
        in("a0") arg0,
   )
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

