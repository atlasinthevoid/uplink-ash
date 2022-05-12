use super::State;
use super::Uuid;

pub fn get_position(state: &mut State, capability: Uuid) {
    let pos = state.get_sibling_by_type(capability, "position".to_string());
    println!("{}, {}, {}", pos.data.float["x"], pos.data.float["y"], pos.data.float["z"]);
}