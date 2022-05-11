use super::Capability;

impl Capability {
    pub fn new_terminal() -> Capability {
        let mut c = Capability::new();
        c.data.string.insert("type".to_string(), "terminal".to_string());
        c.update_commands.push("update_terminal".to_string());
        c
    }
}