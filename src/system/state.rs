use super::HashMap;
use super::Entity;
use super::Uuid;
use super::Capability;
use super::command;
use std::sync::mpsc::Receiver;
use std::io;
use std::sync::mpsc;
use std::{thread};

// Entities are stored by guid
// Capabilities are stored by type for performance
pub struct State {
    pub entities: HashMap<Uuid, Entity>,
    pub capabilities: HashMap<Uuid, Capability>,
    pub by_type: HashMap<String, Vec<Uuid>>,
    pub log: Vec<String>,
    pub stdin_channel: Receiver<String>,
}
impl State {
    pub fn new() -> State {
        State {
            entities: HashMap::new(),
            capabilities: HashMap::new(),
            by_type: HashMap::new(),
            log: Vec::new(),
            stdin_channel: spawn_stdin_channel(),
        }
    }
    pub fn new_capability(&mut self, entity: Uuid, capability: Capability) -> Uuid {
        let id = capability.data.uuid["id"];
        self.entities.get_mut(&entity).unwrap().attach(id);
        
        self.capabilities.insert(id, capability);
        let t = &self.capabilities[&id].data.string["type"];
        if !self.by_type.contains_key(t) {
            self.by_type.insert(t.to_string(), Vec::new());
        }
        self.by_type.get_mut(t).unwrap().push(id);

        //println!("Created new capability {}", self.capabilities[&id].data.string["type"]);
        id
    }
    pub fn get_types(&mut self) -> HashMap<String, Vec<Uuid>> {
        self.by_type.clone()
    }
    pub fn get_capability_commands(&mut self, id: &Uuid) -> Vec<String> {
        self.capabilities[&id].update_commands.clone()
    }
    pub fn command(&mut self, cmd: String, capability: Uuid) {
        match cmd.as_str() {
            "update_terminal" => command::update_terminal::update_terminal(self, capability),
            "status" => self.status(),
            "increment" => command::increment::increment(self, capability),
            "start_vr" => command::vulkan::start_vr::start_vr(self, capability),
            "" => println!("invalid command"),
            _ => println!("invalid command"),
        }
    }
    pub fn status(& self) {
        println!("State contains: ");
        println!("  {} ticks", self.capabilities[&self.by_type["clock"][0]].data.int["ticks"]);
        println!("  {} entities", self.entities.len());
        println!("  {} capabilities", self.capabilities.len());
        println!("  {} types", self.by_type.len());
        println!("  {} log lines", self.log.len());
    }
}

pub fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}