use crate::system::Entity;
use crate::system::capability::Capability;
use crate::system::capability::Update;
use crate::system::capability::Meta;
use crate::system::capability::MetaType;

pub struct Name {
    string: String,
}
impl Name {
    pub fn new(string: String) -> Name {
        Name {
            string,
        }
    }
}
impl MetaType for Name {
    
}
impl Meta<Name> {
    pub fn new(string: String) -> Meta<Name> {
        Meta::base(Name::new(string))
    }
}
impl Capability for Meta<Name> {
    
}
impl Update for Meta<Name> {
    fn update(&mut self) {

    }
}
impl Entity {
    pub fn new_tagged(tag: &str) -> Entity {
        let mut e = Entity::new();
        e.add(Box::new(Meta::<Name>::new(tag.to_string())));
        e
    }
}