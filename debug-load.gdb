set substitute-path /rustc/6ef275e6c3cb1384ec78128eceeb4963ff788dca /home/pillot/.rustup/toolchains/nightly-2019-09-25-x86_64-unknown-linux-gnu/lib/rustlib/src/rust
target extended-remote localhost:3333
monitor reset halt
load
tbreak kmain
