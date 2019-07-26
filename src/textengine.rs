use super::annotation::Annotation;
use super::cas::Cas;
use super::engine;
use super::error::PipelineError;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct PrintEngine {
    pub annotation: String,
}
pub struct Tokenizer();
pub struct RegexEngine {
    pub pattern: String,
    pub annotation: String,
}
pub struct SimpleDocumentReader {
    pub documents: Vec<PathBuf>,
    pub input_dir: String,
    pub document_index: u32,
    pub documents_len: u32,
}

impl engine::Engine for Tokenizer {
    fn process(&self, cas: &mut Cas) -> Result<(), PipelineError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\w+").unwrap();
        }
        for cap in RE.find_iter(cas.text.to_owned().as_str()) {
            let begin = cap.start() as i32;
            let end = cap.end() as i32;
            let annot = Annotation { begin, end };
            cas.insert_annotation("token", annot);
        }
        Ok(())
    }
}
impl engine::Engine for RegexEngine {
    fn process(&self, cas: &mut Cas) -> Result<(), PipelineError> {
        let re: Regex = Regex::new(self.pattern.as_str()).unwrap();
        for cap in re.find_iter(cas.text.to_owned().as_str()) {
            let begin = cap.start() as i32;
            let end = cap.end() as i32;
            let annot = Annotation { begin, end };
            cas.insert_annotation(self.annotation.as_str(), annot);
        }
        Ok(())
    }
}
impl engine::Engine for PrintEngine {
    fn process(&self, cas: &mut Cas) -> Result<(), PipelineError> {
        cas.print_annotations(self.annotation.as_str())?;
        Ok(())
    }
}
impl engine::Reader for SimpleDocumentReader {
    fn execute(&mut self, cas: &mut Cas) -> Result<(), PipelineError> {
        if let Some(pbuf) = self.documents.get(self.document_index as usize) {
            let path = pbuf.as_path();
            if !path.is_dir() {
                let text = fs::read_to_string(path)?;
                cas.text = text;
                self.document_index += 1;
            }
        }
        Ok(())
    }
    fn has_next(&mut self) -> bool {
        self.document_index < self.documents_len
    }
    fn initialize(&mut self) -> Result<(), PipelineError> {
        for entry in WalkDir::new(self.input_dir.as_str())
            .into_iter()
            .filter_map(|e| e.ok())
        {
            self.documents.push(PathBuf::from(entry.path()));
        }
        if self.documents.is_empty() {
            return Err(PipelineError::NoDocumentsFound);
        }
        self.documents_len = self.documents.len() as u32;
        Ok(())
    }
}
