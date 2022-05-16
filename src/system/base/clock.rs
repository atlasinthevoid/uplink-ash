use super::Capability;

impl Capability {
    pub async fn new_clock() -> Capability {
        let mut c = Capability::new().await;
        c.data
            .string
            .insert("type".to_string(), "clock".to_string());
        c.data.int.insert("ticks".to_string(), 0);
        c.update_commands.push("increment".to_string());
        c
    }
}
