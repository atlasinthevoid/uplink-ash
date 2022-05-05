use crate::system::Meta;

// C A P A B I L I T Y
pub trait Capability {
    
}
pub trait Update {
    fn update(&mut self);
}

// N A M E
pub struct Name {
    string: String,
}
impl Name {
    pub fn new(string: String) -> Name {
        println!("{} created", string);
        Name {
            string,
        }
    }
}
impl Capability for Name {
    
}
impl <T> Meta<T> {
    pub fn meta_add_name(&mut self, string: String) {
        self.meta.push(Box::new(Name::new(string)));
    }
}
impl Meta<Name> {
    pub fn new(string: String) -> Meta<Name> {
        Meta {
            meta: Vec::new(),
            data: Name::new(string),
        }
    }
}
impl Update for Meta<Name> {
    fn update(&mut self) {

    }
}

// A U T H O R
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
impl Capability for Author {
    
}
impl Meta<Author> {
    fn new(name: String, id: String) -> Meta<Author> {
        Meta {
            meta: Vec::new(),
            data: Author::new(name, id),
        }
    }
}
impl Update for Meta<Author> {
    fn update(&mut self) {

    }
}

// S T A T E
struct State {
    is_active: bool
}
impl State {
    fn new(is_active: bool) -> State {
        State {
            is_active,
        }
    }
}
impl Capability for State {
    
}
impl Meta<State> {
    fn new(is_active: bool) -> Meta<State> {
        Meta {
            meta: Vec::new(),
            data: State::new(is_active),
        }
    }
}
impl Update for Meta<State> {
    fn update(&mut self) {

    }
}

struct CreationTime {
    time: String,
}
impl CreationTime {
    fn new(time: String) -> CreationTime {
        CreationTime {
            time,
        }
    }
}
impl Capability for CreationTime {
    
}
impl Meta<CreationTime> {
    fn new(time: String) -> Meta<CreationTime> {
        Meta {
            meta: Vec::new(),
            data: CreationTime::new(time),
        }
    }
}
impl Update for Meta<CreationTime> {
    fn update(&mut self) {

    }
}

struct Id {
    id: String,
}
impl Id {
    fn new(id: String) -> Id {
        Id {
            id,
        }
    }
}
impl Capability for Id {

}
impl Meta<Id> {
    fn new(id: String) -> Meta<Id> {
        Meta {
            meta: Vec::new(),
            data: Id::new(id),
        }
    }
}
impl Update for Meta<Id> {
    fn update(&mut self) {

    }
}

struct Parent {
    id: String,
}
impl Parent {
    fn new(id: String) -> Parent {
        Parent {
            id,
        }
    }
}
impl Capability for Parent {
    
}
impl Meta<Parent> {
    fn new(id: String) -> Meta<Parent> {
        Meta {
            meta: Vec::new(),
            data: Parent::new(id),
        }
    }
}
impl Update for Parent {
    fn update(&mut self) {

    }
}