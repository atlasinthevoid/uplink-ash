use uuid::Uuid;
use super::Capability;

impl Capability {
    pub fn new_id() -> Capability {
        let mut c = Capability::new();
        c.data.string.insert("type".to_string(), "id".to_string());
        c.data.uuid.insert("id".to_string(), Uuid::new_v4());
        c
    }
}