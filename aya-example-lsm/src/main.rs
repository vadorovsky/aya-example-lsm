use std::{
    convert::TryInto,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use aya::{programs::Lsm, Bpf};
use aya_log::BpfLogger;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    TermLogger::init(
        LevelFilter::Debug,
        ConfigBuilder::new()
            .set_target_level(LevelFilter::Error)
            .set_location_level(LevelFilter::Error)
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    let code = include_bytes!("../../target/bpfel-unknown-none/debug/aya-example-lsm").to_vec();
    let mut bpf = Bpf::load(&code)?;

    BpfLogger::init(&mut bpf)?;

    let prog: &mut Lsm = bpf.program_mut("lsm_prog_file_open")?.try_into()?;
    prog.load("file_open")?;
    prog.attach()?;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    while running.load(Ordering::SeqCst) {}
    println!("Exiting...");

    Ok(())
}
