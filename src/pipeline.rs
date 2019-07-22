use super::cas::Cas;
use super::engine;

pub fn run(reader: &mut dyn engine::Reader, engines: Vec<Box<engine::Engine>>) {
    reader.initialize();
    while reader.has_next() {
        let mut cas = Cas::new();
        reader.execute(&mut cas);
        for engine in &engines {
            engine.process(&mut cas);
        }
    }
}
