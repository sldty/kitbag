use crate::traits::{
    Address,
    Storable,
};

pub trait Store {
    fn write(&mut self, content: &impl Storable) -> Box<dyn Address>;
    fn read(&mut self, address: &impl Address) -> Box<dyn Storable>;
}
