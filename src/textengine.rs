use std::path::{Path, PathBuf};

use lazy_static::lazy_static;
use regex::Regex;
use walkdir::WalkDir;

use crate::annotation::Annotation;
use crate::cas::Cas;
use crate::engine::{Engine, Reader};
use crate::error::{PipelineError, Result};

#[derive(Debug)]
pub struct PrintEngine {
    annotation: String,
}

#[derive(Debug, Default)]
pub struct SentenceEngine(());

#[derive(Debug, Default)]
pub struct Tokenizer(());

#[derive(Debug)]
pub struct RegexEngine {
    pattern: Regex,
    annotation: String,
}

#[derive(Debug)]
pub struct SimpleDocumentReader {
    documents: std::vec::IntoIter<PathBuf>,
}

impl PrintEngine {
    pub fn new(annotation: String) -> Self {
        Self { annotation }
    }
}

impl SentenceEngine {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Tokenizer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl RegexEngine {
    pub fn new(pattern: Regex, annotation: String) -> Self {
        Self {
            pattern,
            annotation,
        }
    }
}

impl Engine for SentenceEngine {
    fn process(&self, cas: &mut Cas) -> Result<()> {
        lazy_static! {
            static ref RE: RegexEngine =
                RegexEngine::new(Regex::new(r"[^.!?]*[.!?]").unwrap(), "sentence".into());
        }

        RE.process(cas)
    }
}

impl Engine for Tokenizer {
    fn process(&self, cas: &mut Cas) -> Result<()> {
        lazy_static! {
            static ref RE: RegexEngine =
                RegexEngine::new(Regex::new(r"\w+").unwrap(), "token".into());
        }

        RE.process(cas)
    }
}

impl Engine for RegexEngine {
    fn process(&self, cas: &mut Cas) -> Result<()> {
        let annotations: Vec<Annotation> = self
            .pattern
            .find_iter(cas.text())
            .map(|cap| Annotation {
                begin: cap.start(),
                end: cap.end(),
            })
            .collect();

        cas.set_annotations(self.annotation.clone(), annotations);
        Ok(())
    }
}

impl Engine for PrintEngine {
    fn process(&self, cas: &mut Cas) -> Result<()> {
        cas.print_annotations(&self.annotation)
    }
}

impl Reader for SimpleDocumentReader {
    fn next_cas(&mut self) -> Option<Result<Cas>> {
        self.documents.next().map(|path| {
            std::fs::read_to_string(path)
                .map(Cas::new)
                .map_err(PipelineError::IoError)
        })
    }
}

impl SimpleDocumentReader {
    pub fn new(input_dir: &Path) -> Result<Self> {
        let documents: Vec<_> = WalkDir::new(input_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|entry| entry.into_path())
            .filter(|path| !path.is_dir())
            .collect();

        if documents.is_empty() {
            return Err(PipelineError::NoDocumentsFound);
        }

        Ok(Self {
            documents: documents.into_iter(),
        })
    }
}
