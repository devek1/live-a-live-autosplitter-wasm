# Live A Live WASM Autosplitter

A multiplatform autosplitter for Live a Live (PC)

## Supported Features
- Auto start timer on character select
- Auto start timer when selecting NEW GAME
- Game Time through loading removal (not accurate)
- Split on next chapter start (only when timer is running)

# TODO:
- [~] Character Story Splits
   - partially done in `revamp` branch, going through to get scenario ids
- [ ] Full game ending splits
    - [~] True Ending - Sin Odio end flash (this is the main and currently only speedrun category)
    - [ ] Incomplete Destiny Ending - cutscene after completing partial boss rush
    - [ ] Never Ending - defeat Oersted? save prompt after completion? (a change in Scenario Progress after defeating Oersted is not guaranteed, particularly in the context of multi-ending runs like All Achievements)
    - [ ] Sad Ending - skip cutscene after defeating all protagonists
    - [ ] Armageddon (in Oersted route) - animation of activating the Armageddon action
- [ ] TODO: Determine a better way to handle Present Day defeated enemies.
  - intVars + in controller Face icons dont work for last enemy (since the game does this intentionally so you can reload the save)
  - Maybe add a collective split when the odie kill animation starts.
  - [ ] map id change for starting odie.

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
