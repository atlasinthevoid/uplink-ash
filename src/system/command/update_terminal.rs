use super::State;
use super::Uuid;
use std::sync::mpsc::TryRecvError;

pub fn update_terminal(state: &mut State, capability: Uuid) {
    match state.stdin_channel.try_recv() {
        Ok(key) => {
            let mut command = key.to_string();
            command.pop();
            //println!("Received: {}", command);
            state.command(command, capability);
        },
        Err(TryRecvError::Empty) => (),
        Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
    }
}