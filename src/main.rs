mod feature;
mod engine;
use engine::Engine;
mod annotation;
mod cas;
mod textengine;
mod pipeline;

fn main() {
    let regexeng = textengine::RegexAnalysisEngine();
    let regexeng2 = textengine::AnalysisEngine();
    let printeng = textengine::PrintEngine {
        annotation: String::from("regex")
    };
    let mut tengines: Vec<Box<Engine>> = Vec::new();
    tengines.push(Box::new(regexeng));
    tengines.push(Box::new(regexeng2));
    tengines.push(Box::new(printeng));
    let mut reader = textengine::SimpleDocumentReader {
        documents: Vec::new(),
        input_dir: String::from("/home/rafael/rustspace/doctext"),
        document_index: 1,
        documents_len: 0
    };
    pipeline::run(&mut reader, tengines);

}
