use std::fmt;
use std::io;

pub enum PipelineError {
    IoError(io::Error),
    AnnotationMissing,
    NoDocumentsFound,
    InvalidRange,
}

impl fmt::Display for PipelineError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            PipelineError::IoError(ref e) => e.fmt(formatter),
            PipelineError::InvalidRange => formatter.write_str("Invalid value"),
            PipelineError::AnnotationMissing => formatter.write_str("No annotation was found"),
            PipelineError::NoDocumentsFound => formatter.write_str("No documents found"),
        }
    }
}

impl From<io::Error> for PipelineError {
    fn from(err: io::Error) -> PipelineError {
        PipelineError::IoError(err)
    }
}
