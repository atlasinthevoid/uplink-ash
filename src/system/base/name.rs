use super::Capability;

impl Capability {
    pub async fn new_name(string: String) -> Capability {
        let mut c = Capability::new();
        c.data.string.insert("type".to_string(), string);
        c
    }
}