use crate::system::capability::Capability;
use crate::system::capability::Update;
use crate::system::capability::Meta;
use crate::system::capability::MetaType;

pub struct ActiveState {
    is_active: bool
}
impl ActiveState {
    fn new(is_active: bool) -> ActiveState {
        ActiveState {
            is_active,
        }
    }
}
impl MetaType for ActiveState {
    
}
impl Meta<ActiveState> {
    fn new(is_active: bool) -> Meta<ActiveState> {
        Meta::base(ActiveState::new(is_active))
    }
}
impl Capability for Meta<ActiveState> {
    
}
impl Update for Meta<ActiveState> {
    fn update(&mut self) {

    }
}