use std::collections::HashMap;

use crate::annotation::Annotation;
use crate::error::{PipelineError, Result};

#[derive(Clone, Debug, Default)]
pub struct Cas {
    text: String,
    annotations: HashMap<String, Vec<Annotation>>,
}

impl Cas {
    pub fn new(text: String) -> Self {
        Self {
            text,
            annotations: HashMap::new(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_annotations(&mut self, name: String, annotations: Vec<Annotation>) {
        self.annotations.insert(name, annotations);
    }

    pub fn insert_annotation(&mut self, name: String, annotation: Annotation) {
        self.annotations.entry(name).or_default().push(annotation);
    }

    pub fn print_annotations(&self, name: &str) -> Result<()> {
        let annotations = self
            .annotations
            .get(name)
            .ok_or(PipelineError::AnnotationMissing)?;

        for annot in annotations {
            let covered_text = &self.text[annot.begin..annot.end];
            println!(
                "ANNOT {} BEGIN {} END {}\nCOVERED TEXT IS: {}",
                name, annot.begin, annot.end, covered_text
            );
        }

        Ok(())
    }

    pub fn get_covered_annotations(
        &self,
        annot_name: &str,
    ) -> Result<HashMap<String, Vec<&Annotation>>> {
        let mut covered_annotations: HashMap<String, Vec<&Annotation>> = HashMap::new();
        let annot_cover = self
            .annotations
            .get(annot_name)
            .ok_or(PipelineError::AnnotationMissing)?;

        for annot in annot_cover {
            for (name, annotations) in &self.annotations {
                if name != annot_name {
                    covered_annotations
                        .entry(name.clone())
                        .or_default()
                        .extend(annotations.iter().filter(|a| annot.covers(a)));
                }
            }
        }

        Ok(covered_annotations)
    }

    pub fn get_covered_annotations_by(
        &self,
        annot_name: &str,
        annot_covered: &str,
    ) -> Result<HashMap<String, Vec<&Annotation>>> {
        let mut covered_annotations: HashMap<String, Vec<&Annotation>> = HashMap::new();
        let annot_cover = self
            .annotations
            .get(annot_name)
            .ok_or(PipelineError::AnnotationMissing)?;

        for annot in annot_cover {
            if let Some(annotations) = self.annotations.get(annot_covered) {
                covered_annotations
                    .entry(annot_covered.to_string())
                    .or_default()
                    .extend(annotations.iter().filter(|a| annot.covers(a)));
            }
        }

        Ok(covered_annotations)
    }
}
