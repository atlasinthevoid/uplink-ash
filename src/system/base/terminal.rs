use super::Capability;

impl Capability {
    pub async fn new_terminal() -> Capability {
        let mut c = Capability::new().await;
        c.data
            .string
            .insert("type".to_string(), "terminal".to_string());
        c.data.bool.insert("interactive".to_string(), false);
        c.data
            .string
            .insert("interactive".to_string(), String::new());
        c.update_commands.push("update_terminal".to_string());
        c
    }
}
