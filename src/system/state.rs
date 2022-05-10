use super::HashMap;
use super::Entity;
use super::Uuid;
use super::Capability;

// Entities are stored by guid
// Capabilities are stored by type for performance
pub struct State {
    pub entities: HashMap<Uuid, Entity>,
    pub capabilities: HashMap<Uuid, Capability>,
    pub by_type: HashMap<String, Vec<Uuid>>,
    pub log: Vec<String>,
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
        let id = capability.data.uuid["id"];
        self.entities.get_mut(&entity).unwrap().attach(id);
        
        self.capabilities.insert(id, capability);
        let t = &self.capabilities[&id].data.string["type"];
        if !self.by_type.contains_key(t) {
            self.by_type.insert(t.to_string(), Vec::new());
        }
        self.by_type.get_mut(t).unwrap().push(id);

        println!("Created new capability {}", self.capabilities[&id].data.string["type"]);
        id
    }
    pub fn status(& self) {
        println!("State contains: ");
        println!("  {} entities", self.entities.len());
        println!("  {} capabilities", self.capabilities.len());
        println!("  {} types", self.by_type.len());
        println!("  {} log lines", self.log.len());
    }
}