#![no_std]
#![no_main]

use aya_bpf::{macros::lsm, programs::LsmContext};
use aya_log_ebpf::info;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unreachable!()
}

#[lsm(name = "lsm_prog_file_open")]
pub fn lsm_prog_file_open(ctx: LsmContext) -> i32 {
    match unsafe { try_lsm_prog_file_open(ctx) } {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

unsafe fn try_lsm_prog_file_open(ctx: LsmContext) -> Result<i32, i32> {
    info!(&ctx, "a file is opened");

    Ok(0)
}
