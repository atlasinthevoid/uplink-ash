use crate::system::capability::Capability;
use crate::system::capability::Update;
use crate::system::capability::Meta;
use crate::system::capability::MetaType;

pub struct Author {
    name: String,
    id: String,
}
impl Author {
    fn new(name: String, id: String) -> Author {
        Author {
            name,
            id,
        }
    }
}
impl MetaType for Author {
    
}
impl Meta<Author> {
    pub fn new(name: String, id: String) -> Meta<Author> {
        Meta::base(Author::new(name, id))
    }
}
impl Capability for Meta<Author> {
    
}
impl Update for Meta<Author> {
    fn update(&mut self) {

    }
}