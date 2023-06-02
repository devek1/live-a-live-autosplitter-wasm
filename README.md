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

Go to "Edit Splits", and make sure Live a Live (2022) is selected as the game name, and click "Activate" then click "Settings" to change your splits.

To install a new version, restart LiveSplit

## Manual Install

Go to release page and select a release and add Auto Splitting Runtime component and add the file manually.

### For Any% Good Ending Runs

Select Automatic Start on New Game

Select "Start {Chapter name}" for all spits *except* the first characters split, unless you have a "menu split" before that character select.

Note: In the future, preconfigured autosplitter and splits file may be provided in the community discord/src page.

Ending split must be done manually for now.

### For Individual Level Runs

Select Automatic Start on character select

Ending split must be done manually for now.

### NOTE

!!! ENSURE YOU EITHER HAVE YOUR TIMING METHOD ON THE TIMER COMPONENT SET TO `GAME TIME` OR MAKE TWO TIMERS AND SET ONE TO `GAME TIME` !!!

## build

1. install rustup + stable rust https://rustup.rs/
2. install wasm target
  - `rustup target add wasm32-unknown-unknown`
3. build wasm file (--release optional)
  - `cargo build --target wasm32-unknown-unknown --release`
