use super::Capability;

impl Capability {
    pub fn new_terminal() -> Capability {
        let mut c = Capability::new();
        c.data.string.insert("type".to_string(), "terminal".to_string());
        c
    }
}