use super::State;
use super::Uuid;
use super::Capability;

pub async fn create_event(state: &mut State, _capability: Uuid) {
    let event = state.new_entity().await;
    state.new_capability(event, Capability::new_event().await).await;


}