#[derive(Clone, Debug, PartialEq)]
pub enum ErrorKind {
    Validation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModelError {
    pub kind: ErrorKind,
    pub message: String,
}
