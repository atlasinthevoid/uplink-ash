use crate::system::capability::Capability;
use crate::system::capability::Update;
use crate::system::capability::Meta;
use crate::system::capability::MetaType;
use crate::system::State;

pub struct Terminal {
    
}
impl Terminal {
    pub fn new() -> Terminal{
        Terminal {
            
        }
    }
}
impl MetaType for Terminal {
    
}
impl Meta<Terminal> {
    pub fn new() -> Meta<Terminal> {
        Meta::base(Terminal::new())
    }
}
impl Capability for Meta<Terminal> {
    
}
impl Update for Meta<Terminal> {
    fn update(&mut self) {

    }
}