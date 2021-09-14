use std::convert::TryFrom;

use aya::{programs::Lsm, Bpf};

fn main() {
    if let Err(e) = try_main() {
        eprintln!("error: {:#}", e);
    }
}

fn try_main() -> Result<(), anyhow::Error> {
    let code = include_bytes!("../../target/bpfel-unknown-none/debug/aya-example-lsm").to_vec();
    let mut bpf = Bpf::load(&code, None)?;
    let prog = Lsm::try_from(bpf.program_mut("syslog")?)?;
    prog.load();
    // prog.attach()?;

    Ok(())
}
