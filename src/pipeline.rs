use crate::engine::{Engine, Reader};
use crate::error::Result;

pub fn run<R: Reader>(reader: &mut R, engines: &[Box<dyn Engine>]) -> Result<()> {
    while let Some(cas) = reader.next_cas() {
        let mut cas = cas?;
        for engine in engines {
            engine.process(&mut cas)?;
        }
    }
    Ok(())
}
