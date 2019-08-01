use crate::cas::Cas;
use crate::error::Result;

pub trait Engine {
    fn process(&self, cas: &mut Cas) -> Result<()>;
}

pub trait Reader {
    fn next_cas(&mut self) -> Option<Result<Cas>>;
}
