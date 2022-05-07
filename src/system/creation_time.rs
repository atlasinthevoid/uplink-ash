use chrono::DateTime;
use crate::system::capability::Capability;
use crate::system::capability::Update;
use crate::system::capability::Meta;
use crate::system::capability::MetaType;

pub struct CreationTime {
    time: String,
}
impl CreationTime {
    pub fn new() -> CreationTime {
        CreationTime {
            time: "".to_string(),
        }
    }
}
impl MetaType for CreationTime {
    
}
impl Meta<CreationTime> {
    pub fn new() -> Meta<CreationTime> {
        Meta::base(CreationTime::new())
    }
}
impl Capability for Meta<CreationTime> {
    
}
impl Update for Meta<CreationTime> {
    fn update(&mut self) {

    }
}