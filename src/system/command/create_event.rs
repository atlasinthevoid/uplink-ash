use super::State;
use super::Uuid;
use super::Capability;

pub fn create_event(state: &mut State, capability: Uuid) {
    let event = state.new_entity();
    state.new_capability(event, Capability::new_event());


}