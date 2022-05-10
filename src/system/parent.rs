use super::Capability;
use super::Uuid;

impl Capability {
    fn new_parent(id: Uuid) -> Capability {
        let mut c = Capability::new();
        c.data.uuid.insert("parent".to_string(), id);
        c
    }
}