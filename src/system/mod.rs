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

use futures::future;
use std::{thread, time};
use thread::sleep; // 0.3.5

pub async fn init(state: &mut State) {
    init_env(state).await;
    let (_a, _b) = future::join(
        command::start_website::start_website(),
        state.command("start_vr".to_string(), Uuid::new_v4()),
    )
    .await;
}

pub async fn init_env(state: &mut State) {
    println!("Setting up uplink terminal...");
    let terminal = state.new_entity().await;
    state
        .new_capability(terminal, Capability::new_terminal().await)
        .await;
    state
        .new_capability(terminal, Capability::new_position(0f64, 0f64, 0f64).await)
        .await;

    state
        .new_capability(terminal, Capability::new_clock().await)
        .await;

    println!("Initializing default computer...");

    let _user = state.new_entity().await;
    let _linux = state.new_entity().await;
    let _discord = state.new_entity().await;
    let _vscodium = state.new_entity().await;

    println!("Initializing default environment...");

    let _universe = state.new_entity().await;
    let _planet = state.new_entity().await;
    let _chunk = state.new_entity().await;
    let _chunk_hoster = state.new_entity().await;

    //state.status();
}

pub async fn start_game_loop(state: &mut State) {
    loop {
        game_loop(state).await;
    }
}

pub async fn game_loop(state: &mut State) {
    update(state).await;
    sleep(time::Duration::from_millis(100));
}

pub async fn update(state: &mut State) {
    for capabilities in state.get_types().await.values_mut() {
        for capability in capabilities {
            let commands = state.get_capability_commands(capability).await;
            for cmd in commands {
                state.command(cmd.to_string(), *capability).await;
            }
        }
    }
}

pub async fn serialise_game_object() {
    //for (i, arg) in args.iter().enumerate() {
    //
    //}
}
