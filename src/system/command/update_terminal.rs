use super::State;
use super::Uuid;
use std::sync::mpsc::TryRecvError;

pub async fn update_terminal(state: &mut State, capability: Uuid) {
    match state.stdin_channel.try_recv() {
        Ok(key) => {
            let mut command = key.to_string();
            command.pop();
            if !state.capabilities[&capability].data.bool["interactive"] {
                //state.command(command, capability);
            } else {
            }
        }
        Err(TryRecvError::Empty) => (),
        Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
    }
}
