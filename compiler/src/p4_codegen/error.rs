use thiserror::Error;

pub type Result<T, E = CodegenError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum CodegenError {
    #[error("Failed to lookup instruction set")]
    Lookup(#[from] cranelift::codegen::isa::LookupError),
    #[error("Error while using cranelift Module")]
    Module(#[from] cranelift_module::ModuleError),
    #[error("Error while emitting object blob")]
    Object(#[from] cranelift_object::object::write::Error),
}
