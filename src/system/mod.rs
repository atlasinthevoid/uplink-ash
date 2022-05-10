pub mod author;
pub mod capability;
pub mod creation_time;
pub mod id;
pub mod name;
pub mod parent;
pub mod active_state;
pub mod terminal;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use uuid::Uuid;
use capability::Capability;

pub struct Entity {
    capabilities: Vec<Uuid>,
}
impl Entity {
    pub fn new() -> Entity {
        Entity {
            capabilities: Vec::new(),
        }
    }
    pub fn attach(&mut self, value: Uuid){
        //self.capabilities.push(value)
    }
}
impl State {
    pub fn new_entity(&mut self) -> Uuid {
        let id = Capability::new_id();
        let uuid = id.data.uuid["id"];
        self.entities.insert(uuid, Entity::new());
        self.new_capability(uuid, id);
        self.new_capability(uuid, Capability::new_creation_time());

        match uuid.to_string().split("-").next() {
            Some(x) => {
                println!("Created new entity {}", x);
            }
            None => {
                println!("Created new entity <INVALID ID>");
            }
        }
                
        uuid
    }
}

pub fn init(state: &mut State) {
    init_env(state);
}

pub fn init_env(state: &mut State) {
    println!("Initializing default environment...");
    let computer = state.new_entity();
    state.new_capability(computer, Capability::new_terminal());

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
    capabilities: HashMap<Uuid, Capability>,
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
    pub fn new_capability(&mut self, entity: Uuid, capability: Capability) -> Uuid {
        //let x = capability.data();
        //println!("Created new capability {}", capability.data.string["name"]);
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