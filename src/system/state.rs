use super::command;
use super::Capability;
use super::Entity;
use super::HashMap;
use super::Uuid;
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

// Entities are stored by guid
// Capabilities are stored by type for performance
pub struct State {
    pub entities: HashMap<Uuid, Entity>,
    pub capabilities: HashMap<Uuid, Capability>,
    pub by_type: HashMap<String, Vec<Uuid>>,
    pub get_entity: HashMap<Uuid, Uuid>,
    pub log: Vec<String>,
    pub stdin_channel: Receiver<String>,
}
impl State {
    pub async fn new() -> State {
        State {
            entities: HashMap::new(),
            capabilities: HashMap::new(),
            by_type: HashMap::new(),
            get_entity: HashMap::new(),
            log: Vec::new(),
            stdin_channel: spawn_stdin_channel().await,
        }
    }
    pub async fn new_capability(&mut self, entity: Uuid, capability: Capability) -> Uuid {
        let id = capability.data.uuid["id"];
        self.capabilities.insert(id, capability);
        let t = &self.capabilities[&id].data.string["type"];
        self.entities
            .get_mut(&entity)
            .unwrap()
            .attach(id, t.to_string())
            .await;

        if !self.by_type.contains_key(t) {
            self.by_type.insert(t.to_string(), Vec::new());
        }
        self.by_type.get_mut(t).unwrap().push(id);

        self.get_entity.insert(id, entity);

        //println!("Created new capability {}", self.capabilities[&id].data.string["type"]);
        id
    }
    pub async fn get_types(&mut self) -> HashMap<String, Vec<Uuid>> {
        self.by_type.clone()
    }
    pub async fn get_capability_commands(&mut self, id: &Uuid) -> Vec<String> {
        self.capabilities[&id].update_commands.clone()
    }
    pub async fn command(&mut self, cmd: String, capability: Uuid) {
        match cmd.as_str() {
            "update_terminal" => command::update_terminal::update_terminal(self, capability).await,
            "status" => self.status().await,
            "increment" => command::increment::increment(self, capability).await,
            "start_vr" => command::vulkan::start_vr::start_vr(self, capability).await,
            "get_position" => command::get_position::get_position(self, capability).await,
            "get_chunk_position" => {
                command::get_chunk_position::get_chunk_position(self, capability).await
            }
            //"start_website" => command::start_website::start_website(self, capability),
            //"start_vulkan_cube" => command::vulkan::start_vulkan_cube(self, capability),
            "create_event" => command::create_event(self, capability).await,
            "" => println!("invalid command"),
            _ => println!("invalid command"),
        }
    }
    pub async fn status(&self) {
        println!("State contains: ");
        println!(
            "  {} ticks",
            self.capabilities[&self.by_type["clock"][0]].data.int["ticks"]
        );
        println!("  {} entities", self.entities.len());
        println!("  {} capabilities", self.capabilities.len());
        println!("  {} types", self.by_type.len());
        println!("  {} log lines", self.log.len());
    }
    pub async fn get_sibling_by_type(&self, capability: Uuid, t: String) -> &Capability {
        &self.capabilities[&self.entities[&self.get_entity[&capability]].by_type[&t][0]]
    }
}

pub async fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}
