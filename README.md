# aya-example-lsm

## Prerequisites

1. Install a rust stable toolchain: `rustup install stable`
1. Install a rust nightly toolchain: `rustup install nightly`
1. Install bpf-linker: `cargo +nightly install --git https://github.com/alessandrod/bpf-linker bpf-linker --no-default-features --features rust-llvm --force`

## Build eBPF

```bash
cargo xtask build-ebpf
```

To perform a release build you can use the `--release` flag.
You may also change the target architecture with the `--target` flag

## Build Userspace

```bash
cargo build
```

## Run

```bash
./target/debug/aya-example-lsm
```
