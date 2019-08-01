use regex::Regex;

use rust_pipeline::{
    engine::Engine,
    pipeline,
    textengine::{PrintEngine, RegexEngine, SentenceEngine, SimpleDocumentReader, Tokenizer},
};

fn main() {
    let input_dir = std::env::args().nth(1).expect("No directory provided");

    let mut reader = SimpleDocumentReader::new(input_dir.as_ref()).unwrap();

    let tengines: [Box<dyn Engine>; 4] = [
        Box::new(Tokenizer::new()),
        Box::new(SentenceEngine::new()),
        Box::new(RegexEngine::new(
            Regex::new(r"\d{2}").unwrap(),
            String::from("number"),
        )),
        Box::new(PrintEngine::new(String::from("number"))),
    ];

    match pipeline::run(&mut reader, &tengines) {
        Ok(()) => println!("Sucess"),
        Err(e) => println!("Error {}", e),
    }
}
