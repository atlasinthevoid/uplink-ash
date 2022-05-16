use super::Display;
use super::Formatter;
use super::HashMap;
use super::Result;
use super::Uuid;
use chrono::{DateTime, Utc};

pub struct Capability {
    pub data: Data,
    pub update_commands: Vec<String>,
}
impl Capability {
    pub async fn new() -> Capability {
        let mut c = Capability {
            data: Data::new(),
            update_commands: Vec::new(),
        };
        c.data.uuid.insert("id".to_string(), Uuid::new_v4());
        c.data
            .date_time
            .insert("creation time".to_string(), Utc::now());
        c
    }
}
impl Display for Capability {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} <{:?}>", self.data, self.update_commands)
    }
}

pub struct Data {
    pub int: HashMap<String, u64>,
    pub signed_int: HashMap<String, i64>,
    pub float: HashMap<String, f64>,
    pub bool: HashMap<String, bool>,
    pub string: HashMap<String, String>,
    pub uuid: HashMap<String, Uuid>,
    pub date_time: HashMap<String, DateTime<Utc>>,

    pub int_array: HashMap<String, Vec<u64>>,
    pub signed_int_array: HashMap<String, Vec<i64>>,
    pub float_array: HashMap<String, Vec<f64>>,
    pub bool_array: HashMap<String, Vec<bool>>,
    pub string_array: HashMap<String, Vec<String>>,
    pub uuid_array: HashMap<String, Vec<Uuid>>,
    pub date_time_array: HashMap<String, DateTime<Utc>>,
}
impl Data {
    fn new() -> Data {
        Data {
            int: HashMap::new(),
            signed_int: HashMap::new(),
            float: HashMap::new(),
            bool: HashMap::new(),
            string: HashMap::new(),
            uuid: HashMap::new(),
            date_time: HashMap::new(),

            int_array: HashMap::new(),
            signed_int_array: HashMap::new(),
            float_array: HashMap::new(),
            bool_array: HashMap::new(),
            string_array: HashMap::new(),
            uuid_array: HashMap::new(),
            date_time_array: HashMap::new(),
        }
    }
}
impl Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
            self.int,
            self.signed_int,
            self.float,
            self.bool,
            self.string,
            self.uuid,
            self.date_time,
            self.int_array,
            self.signed_int_array,
            self.float_array,
            self.bool_array,
            self.string_array,
            self.uuid_array,
            self.date_time_array
        )
    }
}
