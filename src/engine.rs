use super::cas::Cas;

pub trait Engine {
    fn process(&self, cas: &mut Cas);
}

pub trait Reader {
    fn has_next(&mut self) -> bool;
    fn execute(&mut self, cas: &mut Cas);
    fn initialize(&mut self);
}
