use super::cas::Cas;
use super::error::PipelineError;

pub trait Engine {
    fn process(&self, cas: &mut Cas) -> Result<(), PipelineError>;
}

pub trait Reader {
    fn has_next(&mut self) -> bool;
    fn execute(&mut self, cas: &mut Cas) -> Result<(), PipelineError>;
    fn initialize(&mut self) -> Result<(), PipelineError>;
}
