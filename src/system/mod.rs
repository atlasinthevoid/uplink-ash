pub mod author;
pub mod capability;
pub mod creation_time;
pub mod id;
pub mod name;
pub mod parent;
pub mod active_state;
pub mod terminal;
use std::collections::HashMap;
use uuid::Uuid;
use capability::Capability;
use capability::Meta;
use terminal::Terminal;
use id::Id;
use creation_time::CreationTime;

pub struct Entity {
    capabilities: Vec<uuid::Uuid>,
}
impl Entity {
    pub fn new() -> Entity {
        Entity {
            capabilities: Vec::new(),
        }
    }
    pub fn add(&mut self, value: Box<dyn Capability>){
        //self.capabilities.push(value)
    }
}
impl State {
    pub fn new_entity(&mut self) -> Uuid {
        let id = Meta::<Id>::new();
        let entity = Entity::new();
        //self.entities.insert(id, entity);
        //self.new_capability(id, Box::new());
        //self.new_capability(id, Box::new(Meta::<CreationTime>::new()));
        
        id.data.id
    }
}

pub fn init(state: &mut State) {
    init_env(state);
}

pub fn init_env(state: &mut State) {
    println!("Initializing default environment...");
    let computer = state.new_entity();
    state.new_capability(computer, Box::new(Meta::<Terminal>::new()));

    let user = state.new_entity();
    let linux = state.new_entity();
    let discord = state.new_entity();
    let vscodium = state.new_entity();
}
pub fn start_game_loop(state: &mut State) {
    loop {
        game_loop(state);
    }
}

pub fn game_loop(state: &mut State) {

}

// Entities are stored by guid
// Capabilities are stored by type for performance
pub struct State {
    entities: HashMap<Uuid, Entity>,
    capabilities: HashMap<Uuid, Box<dyn Capability>>,
    by_type: HashMap<String, Vec<Uuid>>,
    log: Vec<String>,
}
impl State {
    pub fn new() -> State {
        State {
            entities: HashMap::new(),
            capabilities: HashMap::new(),
            by_type: HashMap::new(),
            log: Vec::new(),
        }
    }
    pub fn new_capability(&mut self, entity: Uuid, capability: Box<dyn Capability>) -> Uuid {
        Uuid::new_v4()
    }
}
pub fn serialise_game_object(){
    //for (i, arg) in args.iter().enumerate() {
    //
    //}
}

pub fn register_component() {

}

pub fn register_entity() {

}