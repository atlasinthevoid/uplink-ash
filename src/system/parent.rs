use crate::system::capability::Capability;
use crate::system::capability::Update;
use crate::system::capability::Meta;
use crate::system::capability::MetaType;

pub struct Parent {
    id: String,
}
impl Parent {
    fn new(id: String) -> Parent {
        Parent {
            id,
        }
    }
}
impl MetaType for Parent {
    
}
impl Meta<Parent> {
    fn new(id: String) -> Meta<Parent> {
        Meta::base(Parent::new(id))
    }
}
impl Capability for Meta<Parent> {
    
}
impl Update for Meta<Parent> {
    fn update(&mut self) {

    }
}