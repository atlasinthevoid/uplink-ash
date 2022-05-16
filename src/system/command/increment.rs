use super::State;
use super::Uuid;

pub async fn increment(state: &mut State, capability: Uuid) {
    let current = state.capabilities[&capability].data.int["ticks"];
    state
        .capabilities
        .get_mut(&capability)
        .unwrap()
        .data
        .int
        .insert("ticks".to_string(), current + 1);
}
