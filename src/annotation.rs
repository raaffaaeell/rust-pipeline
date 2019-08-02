#[derive(Clone, Debug)]
pub struct Annotation {
    pub begin: usize,
    pub end: usize,
}

impl Annotation {
    pub fn covers(&self, other: &Self) -> bool {
        self.begin <= other.begin && other.end <= self.end
    }
}
