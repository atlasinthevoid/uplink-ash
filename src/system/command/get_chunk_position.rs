use super::State;
use super::Uuid;

pub async fn get_chunk_position(state: &mut State, capability: Uuid) {
    let pos = state
        .get_sibling_by_type(capability, "position".to_string())
        .await;
    let x = pos.data.float["x"];
    let y = pos.data.float["y"];
    let z = pos.data.float["z"];

    let size: f64 = 8f64;
    let zero: f64 = 0f64;
    let mut chunk_x: f64;
    let mut chunk_y: f64;
    let mut chunk_z: f64;
    if x / size > zero {
        chunk_x = (x / size).floor();
    } else {
        chunk_x = (x / size).ceil();
    }

    // Fix for negitive 0 (-0.0)
    if chunk_x == zero {
        chunk_x = zero;
    }

    if y / size > zero {
        chunk_y = (y / size).floor();
    } else {
        chunk_y = (y / size).ceil();
    }

    // Fix for negitive 0 (-0.0)
    if chunk_y == zero {
        chunk_y = zero;
    }

    if z / size > zero {
        chunk_z = (z / size).floor();
    } else {
        chunk_z = (z / size).ceil();
    }

    // Fix for negitive 0 (-0.0)
    if chunk_z == zero {
        chunk_z = zero;
    }

    println!("{}, {}, {}", chunk_x, chunk_y, chunk_z);
}
