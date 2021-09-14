#![no_std]
#![no_main]

use aya_bpf::macros::lsm;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unreachable!()
}

#[lsm(name = "syslog")]
fn syslog_audit(_ctx: aya_bpf::programs::LsmContext) -> i32 {
    return 0;
}
