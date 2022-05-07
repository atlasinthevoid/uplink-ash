use super::Id;
use super::CreationTime;

pub struct Meta<T> {
    meta: Vec<Box<dyn MetaType>>,
    data: T,
}
impl <T> Meta<T> {
    pub fn base(data: T) -> Meta<T> {
        let mut m = Meta {
            meta: Vec::new(),
            data
        };
        m.add(Box::new(Id::new()));
        m.add(Box::new(CreationTime::new()));
        m
    }
    pub fn add(&mut self, value: Box<dyn MetaType>) {
        self.meta.push(value);
    }
}
pub trait MetaType {

}

pub trait Capability {
    
}
pub trait Update {
    fn update(&mut self);
}