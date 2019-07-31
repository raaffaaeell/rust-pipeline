use super::annotation::Annotation;
use super::cas::Cas;
use super::engine;
use super::error::PipelineError;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct PrintEngine {
    pub annotation: String,
}

pub struct SentenceEngine();

pub struct Tokenizer();

pub struct RegexEngine {
    pub pattern: Regex,
    pub annotation: String,
}

pub struct SimpleDocumentReader {
    pub documents: Vec<PathBuf>,
    pub input_dir: String,
    pub document_index: u32,
    pub documents_len: u32,
}

impl engine::Engine for SentenceEngine {
    fn process(&self, cas: &mut Cas) -> Result<(), PipelineError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[^.!?]*[.!?]").unwrap();
        }
        let mut annotations: Vec<Annotation> = Vec::new();
        for cap in RE.find_iter(cas.text.as_str()) {
            let begin = cap.start() as i32;
            let end = cap.end() as i32;
            let annot = Annotation { begin, end };
            annotations.push(annot);
        }
        cas.insert_annotations("sentence", annotations);
        Ok(())
    }
}

impl engine::Engine for Tokenizer {
    fn process(&self, cas: &mut Cas) -> Result<(), PipelineError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\w+").unwrap();
        }
        let mut annotations: Vec<Annotation> = Vec::new();
        for cap in RE.find_iter(cas.text.as_str()) {
            let begin = cap.start() as i32;
            let end = cap.end() as i32;
            let annot = Annotation { begin, end };
            annotations.push(annot);
        }
        cas.insert_annotations("token", annotations);
        Ok(())
    }
}

impl engine::Engine for RegexEngine {
    fn process(&self, cas: &mut Cas) -> Result<(), PipelineError> {
        let mut annotations: Vec<Annotation> = Vec::new();
        for cap in self.pattern.find_iter(cas.text.as_str()) {
            let begin = cap.start() as i32;
            let end = cap.end() as i32;
            let annot = Annotation { begin, end };
            annotations.push(annot);
        }
        cas.insert_annotations(self.annotation.as_str(), annotations);
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
                cas.text = fs::read_to_string(path)?;
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
