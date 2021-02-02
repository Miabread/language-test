use thiserror::Error;

pub type Result<T, E = BodyError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum BodyError {
    #[error("Tried to call unknown function `{name}`")]
    UnknownCall { name: String },
}
