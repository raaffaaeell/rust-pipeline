use std::{fmt, io};

pub type Result<T, E = PipelineError> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum PipelineError {
    IoError(io::Error),
    AnnotationMissing,
    NoDocumentsFound,
    InvalidRange,
}

impl fmt::Display for PipelineError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineError::IoError(e) => e.fmt(formatter),
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
