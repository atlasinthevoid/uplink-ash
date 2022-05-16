use super::Capability;
use chrono::Utc;

impl Capability {
    pub async fn new_event() -> Capability {
        let mut c = Capability::new().await;
        c.data
            .string
            .insert("type".to_string(), "event".to_string());
        c.data.string.insert("title".to_string(), String::new());
        c.data.string.insert("tag".to_string(), String::new());
        c.data.date_time.insert("start".to_string(), Utc::now());
        c.data.date_time.insert("end".to_string(), Utc::now());
        c.data.string.insert("repeat".to_string(), String::new());
        c.data.string_array.insert("with".to_string(), Vec::new());
        c.data.string.insert("location".to_string(), String::new());
        c.data
            .string_array
            .insert("notifications".to_string(), Vec::new());
        c.data.string.insert("color".to_string(), String::new());
        c.data
            .string
            .insert("description".to_string(), String::new());
        c.data
            .uuid_array
            .insert("attachments".to_string(), Vec::new());
        c
    }
}
