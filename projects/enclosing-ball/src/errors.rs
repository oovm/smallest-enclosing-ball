#[derive(Debug, Copy, Clone)]
pub enum EnclosingError {
    Insufficient { require: usize, points: usize },
}

pub type Result<T> = std::result::Result<T, EnclosingError>;
