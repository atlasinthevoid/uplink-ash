use super::Capability;

impl Capability {
    fn new_active_state(is_active: bool) -> Capability {
        let mut c = Capability::new();
        c.data.bool.insert("state".to_string(), is_active);
        c
    }
}