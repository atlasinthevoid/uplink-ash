use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
pub use uuid::Uuid;

pub mod base;

pub mod entity;
pub use entity::Entity;

pub mod state;
pub use state::State;

pub mod capability;
pub use capability::Capability;

pub mod command;
pub use command::raw_log;

use std::{thread, time};
use thread::sleep;

pub async fn init(state: &mut State) {
    init_env(state);
    state.command("start_vulkan_cube".to_string(), Uuid::new_v4());
    //let future = command::start_website::start_website();
    //future.await;
}

pub fn init_env(state: &mut State) {
    println!("Setting up uplink terminal...");
    let terminal = state.new_entity();
    state.new_capability(terminal, Capability::new_terminal());
    state.new_capability(terminal, Capability::new_position(0f64, 0f64, 0f64));

    state.new_capability(terminal, Capability::new_clock());

    println!("Initializing default computer...");

    let _user = state.new_entity();
    let _linux = state.new_entity();
    let _discord = state.new_entity();
    let _vscodium = state.new_entity();

    println!("Initializing default environment...");

    let _universe = state.new_entity();
    let _planet = state.new_entity();
    let _chunk = state.new_entity();
    let _chunk_hoster = state.new_entity();

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