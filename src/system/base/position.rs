use super::Capability;

impl Capability {
    pub async fn new_position(x: f64, y: f64, z: f64) -> Capability {
        let mut c = Capability::new().await;
        c.data
            .string
            .insert("type".to_string(), "position".to_string());
        c.data.float.insert("x".to_string(), x);
        c.data.float.insert("y".to_string(), y);
        c.data.float.insert("z".to_string(), z);
        c
    }
}
