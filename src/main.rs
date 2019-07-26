mod engine;
mod feature;
use engine::Engine;
mod annotation;
mod cas;
mod error;
mod pipeline;
mod textengine;
#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn main() {
    let tokenizer = textengine::Tokenizer();
    let re = Regex::new(r"\d").unwrap();
    let regexeng = textengine::RegexEngine {
        annotation: String::from("numero"),
        pattern: re,
    };
    let printeng = textengine::PrintEngine {
        annotation: String::from("numero"),
    };
    let mut tengines: Vec<Box<Engine>> = Vec::new();
    tengines.push(Box::new(tokenizer));
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
