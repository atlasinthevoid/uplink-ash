use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use uuid::Uuid;

pub mod id;
pub mod creation_time;
pub mod terminal;
pub mod clock;

pub mod entity;
pub use entity::Entity;

pub mod state;
pub use state::State;

pub mod capability;
use capability::Capability;

pub mod command;
pub use command::raw_log;

use std::{thread, time};
use thread::sleep;

pub fn init(state: &mut State) {
    init_env(state);
}

pub fn init_env(state: &mut State) {
    //println!("Setting up terminal...");
    let terminal = state.new_entity();
    state.new_capability(terminal, Capability::new_terminal());

    state.new_capability(terminal, Capability::new_clock());

    //println!("Initializing default environment...");

    let user = state.new_entity();
    let linux = state.new_entity();
    let discord = state.new_entity();
    let vscodium = state.new_entity();

    //state.status();
}
pub fn start_game_loop(state: &mut State) {
    loop {
        game_loop(state);
    }
}

pub fn game_loop(state: &mut State) {
    update(state);
    sleep(time::Duration::from_millis(100));
}

pub fn update(state: &mut State) {
    for capabilities in state.get_types().values_mut() {
        for capability in capabilities {
            let commands = state.get_capability_commands(capability);
            for cmd in commands {
                state.command(cmd.to_string(), *capability);
            }
        }
    }
}

pub fn serialise_game_object(){
    //for (i, arg) in args.iter().enumerate() {
    //
    //}
}