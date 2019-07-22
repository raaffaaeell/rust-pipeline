use super::annotation::Annotation;
use super::cas::Cas;
use super::engine;
use regex::Regex;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct PrintEngine {
    pub annotation: String,
}
pub struct AnalysisEngine();
pub struct RegexAnalysisEngine();
pub struct SimpleDocumentReader {
    pub documents: Vec<PathBuf>,
    pub input_dir: String,
    pub document_index: u32,
    pub documents_len: u32,
}

impl engine::Engine for AnalysisEngine {
    fn process(&self, cas: &mut Cas) {
        let re = Regex::new(r"\d{2}").unwrap();
        let textstring = cas.text.clone();
        for cap in re.find_iter(textstring.as_str()) {
            let begin = cap.start() as i32;
            let end = cap.end() as i32;
            let annot = Annotation { begin, end };
            cas.insert_annotation("regex3", annot);
        }
    }
}
impl engine::Engine for RegexAnalysisEngine {
    fn process(&self, cas: &mut Cas) {
        let re = Regex::new(r"\d{4}").unwrap();
        let textstring = cas.text.clone();
        for cap in re.find_iter(textstring.as_str()) {
            let begin = cap.start() as i32;
            let end = cap.end() as i32;
            let annot = Annotation { begin, end };
            cas.insert_annotation("regex", annot);
        }
    }
}
impl engine::Engine for PrintEngine {
    fn process(&self, cas: &mut Cas) {
        cas.print_annotations(self.annotation.as_str());
    }
}
impl engine::Reader for SimpleDocumentReader {
    fn execute(&mut self, cas: &mut Cas) {
        let pbuf = self.documents.get(self.document_index as usize).unwrap();
        let path = pbuf.as_path();
        if !path.is_dir() {
            let text = fs::read_to_string(path).expect("erro ao ler arquivo");
            cas.text = text;
            self.document_index += 1;
        }
    }
    fn has_next(&mut self) -> bool {
        self.document_index < self.documents_len
    }
    fn initialize(&mut self) {
        let path = Path::new(&self.input_dir);
        let len = WalkDir::new(path).into_iter().count() as u32;
        self.documents_len = len;

        for entry in WalkDir::new(self.input_dir.as_str())
            .into_iter()
            .filter_map(|e| e.ok())
        {
            self.documents.push(PathBuf::from(entry.path()));
        }
    }
}
