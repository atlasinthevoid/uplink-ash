use super::Capability;
use uuid::Uuid;

impl Capability {
    pub async fn new_id() -> Capability {
        let mut c = Capability::new().await;
        c.data.string.insert("type".to_string(), "id".to_string());
        c.data.uuid.insert("id".to_string(), Uuid::new_v4());
        c
    }
}
