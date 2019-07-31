use regex::Regex;
use rust_pipeline::{engine::Engine, pipeline, textengine};

fn main() {
    let tokenizer = textengine::Tokenizer();
    let sentence = textengine::SentenceEngine();
    let re = Regex::new(r"\d{2}").unwrap();
    let regexeng = textengine::RegexEngine {
        annotation: String::from("number"),
        pattern: re,
    };
    let printeng = textengine::PrintEngine {
        annotation: String::from("number"),
    };
    let mut tengines: Vec<Box<dyn Engine>> = Vec::new();
    tengines.push(Box::new(tokenizer));
    tengines.push(Box::new(sentence));
    tengines.push(Box::new(regexeng));
    tengines.push(Box::new(printeng));
    let mut reader = textengine::SimpleDocumentReader {
        documents: Vec::new(),
        input_dir: String::from("/home/rafael/rustspace/doctext"),
        document_index: 1,
        documents_len: 0,
    };
    match pipeline::run(&mut reader, tengines) {
        Ok(()) => println!("Sucess"),
        Err(e) => println!("Error {}", e),
    }
}
