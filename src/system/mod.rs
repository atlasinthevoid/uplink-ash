use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use uuid::Uuid;

pub mod id;
pub mod creation_time;
pub mod terminal;

pub mod entity;
pub use entity::Entity;

pub mod state;
pub use state::State;

pub mod capability;
use capability::Capability;

pub fn init(state: &mut State) {
    init_env(state);
}

pub fn init_env(state: &mut State) {
    println!("Initializing default environment...");
    let computer = state.new_entity();
    state.new_capability(computer, Capability::new_terminal());

    let user = state.new_entity();
    let linux = state.new_entity();
    let discord = state.new_entity();
    let vscodium = state.new_entity();

    state.status();
}
pub fn start_game_loop(state: &mut State) {
    loop {
        game_loop(state);
    }
}

pub fn game_loop(state: &mut State) {

}

pub fn serialise_game_object(){
    //for (i, arg) in args.iter().enumerate() {
    //
    //}
}