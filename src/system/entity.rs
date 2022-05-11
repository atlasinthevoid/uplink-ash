use super::Uuid;
use super::State;
use super::Capability;

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
        self.capabilities.push(value)
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
                //println!("Created new entity {}", x);
            }
            None => {
                //println!("Created new entity <INVALID ID>");
            }
        }
                
        uuid
    }
}