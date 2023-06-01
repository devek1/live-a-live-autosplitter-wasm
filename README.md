# Live A Live WASM Autosplitter

A multiplatform autosplitter for Live a Live (PC)

## Supported Features
- Auto start timer on character select
- Auto start timer when selecting NEW GAME
- Game Time through loading removal (not accurate)
- Split on next chapter start (only when timer is running)

# TODO:
- [ ] Character Story Splits (will look more into this at a later time)
- [ ] Full game ending splits
    - [ ] final boss end flash (this is going to be a nightmare)


## Install

Since this autosplitter is in prerelease, you'll need to download the following file and add an "Auto Splitting Runtime" to your layout and add this file. WASM files should be fully supported in a future version of livesplit.

### Windows

https://github.com/Eein/live-a-live-autosplitter-wasm/releases/latest/download/live_a_live_autosplitter_wasm.wasm

## build
1. install rustup + stable rust https://rustup.rs/
2. install wasm target
  - `rustup target add wasm32-unknown-unknown`
3. build wasm file (--release optional)
  - `cargo build --target wasm32-unknown-unknown --release`
