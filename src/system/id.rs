use uuid::Uuid;
use crate::system::capability::Capability;
use crate::system::capability::Update;
use crate::system::capability::Meta;
use crate::system::capability::MetaType;

pub struct Id {
    id: Uuid,
}
impl Id {
    pub fn new() -> Id {
        Id {
            id: Uuid::new_v4(),
        }
    }
}
impl MetaType for Id {
    
}
impl Meta<Id> {
    pub fn new() -> Meta<Id> {
        Meta::base(Id::new())
    }
}
impl Capability for Meta<Id> {

}
impl Update for Meta<Id> {
    fn update(&mut self) {

    }
}