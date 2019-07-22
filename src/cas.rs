use super::annotation::Annotation;
use std::collections::HashMap;

pub struct Cas {
    pub text: String,
    pub annotations: HashMap<String, Vec<Annotation>>,
}

impl Cas {
    pub fn new() -> Cas {
        Cas {
            text: String::from(""),
            annotations: HashMap::new(),
        }
    }
    pub fn get_covered_text(self: &Self, begin: usize, end: usize) -> &str {
        let ref_text: &str = self.text.as_str();
        &ref_text[begin..end]
    }
    pub fn get_covered_text_safe(self: &Self, begin: usize, end: usize) -> Result<&str, String> {
        let len = self.text.len();
        if len < end {
            return Err("End value invalid".to_string());
        }
        let ref_text: &str = self.text.as_str();
        Ok(&ref_text[begin..end])
    }

    pub fn insert_annotations(&mut self, name: &str, annotations: Vec<Annotation>) {
        self.annotations.insert(name.to_string(), annotations);
    }
    pub fn insert_annotation(&mut self, name: &str, annotation: Annotation) {
        self.annotations
            .entry(name.to_string())
            .or_insert(Vec::new())
            .push(annotation);
    }
    pub fn print_annotations(&self, name: &str) {
        let annotations = self.annotations.get(name).unwrap();
        for annot in annotations {
            let covered_text = self
                .get_covered_text_safe(annot.begin as usize, annot.end as usize)
                .unwrap();
            println!(
                "ANNOT {} BEGIN {} END {}\nCOVERED TEXT IS: {}",
                name, annot.begin, annot.end, covered_text
            );
        }
    }
}
