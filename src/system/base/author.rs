use super::Capability;
use super::Uuid;

impl Capability {
    fn new_author(name: String, id: Uuid) -> Capability {
        let mut c = Capability::new();
        c.data.uuid.insert("author".to_string(), id);
        c.data.string.insert("author".to_string(), name);
        c
    }
}