pub mod capability;

pub struct Meta<T> {
    meta: Vec<Box<dyn capability::Capability>>,
    data: T,
}
impl <T> Meta<T> {
    pub fn meta_add(&mut self, value: Box<dyn capability::Capability>) {
        self.meta.push(value);
    }
}

pub struct Entity {
    capabilities: Vec<Meta<Box<dyn capability::Capability>>>,
}
impl Entity {
    pub fn new() -> Entity{
        Entity {
            capabilities: Vec::new(),
        }
    }
}
impl Meta<Entity> {
    pub fn new() -> Meta<Entity>{
        Meta {
            meta: Vec::new(),
            data: Entity::new(),
        }
    }
    
    pub fn add(&mut self, value: Meta<Box<dyn capability::Capability>>){
        //value.meta.push();
        self.data.capabilities.push(value)
    }
}

pub fn init_env() {
    println!("Initializing default environment...");

    let mut e = Meta::<Entity>::new();
    e.meta_add_name(String::from("computer"));

    let mut e = Meta::<Entity>::new();
    e.meta_add_name(String::from("user"));

    let mut e = Meta::<Entity>::new();
    e.meta_add_name(String::from("linux"));

    let mut e = Meta::<Entity>::new();
    e.meta_add_name(String::from("discord"));

    let mut e = Meta::<Entity>::new();
    e.meta_add_name(String::from("vscodium"));

    let mut e = Meta::<Entity>::new();
    e.meta_add_name(String::from("filesystem"));
}