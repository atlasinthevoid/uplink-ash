use super::State;
use super::Uuid;

pub async fn log(_state: &mut State, _capability: Uuid) {}

pub async fn raw_log(_msg: String) {}
