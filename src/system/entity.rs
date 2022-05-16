use super::Capability;
use super::HashMap;
use super::State;
use super::Uuid;

pub struct Entity {
    pub capabilities: Vec<Uuid>,
    pub by_type: HashMap<String, Vec<Uuid>>,
}
impl Entity {
    pub async fn new() -> Entity {
        Entity {
            capabilities: Vec::new(),
            by_type: HashMap::new(),
        }
    }
    pub async fn attach(&mut self, value: Uuid, t: String) {
        self.capabilities.push(value);
        if !self.by_type.contains_key(&t) {
            self.by_type.insert(t.to_string(), Vec::new());
        }
        self.by_type.get_mut(&t).unwrap().push(value);
    }
}
impl State {
    pub async fn new_entity(&mut self) -> Uuid {
        let id = Capability::new_id().await;
        let uuid = id.data.uuid["id"];
        self.entities.insert(uuid, Entity::new().await);
        self.new_capability(uuid, id).await;
        self.new_capability(uuid, Capability::new_creation_time().await)
            .await;

        match uuid.to_string().split("-").next() {
            Some(_x) => {
                //println!("Created new entity {}", x);
            }
            None => {
                //println!("Created new entity <INVALID ID>");
            }
        }

        uuid
    }
}
