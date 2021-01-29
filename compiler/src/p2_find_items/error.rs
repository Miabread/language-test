use thiserror::Error;

pub type Result<T, E = FindItemsError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum FindItemsError {
    #[error("Local {item_ty} item `{name}` must have a definition. Maybe you meant to put it inside an external?")]
    LocalItemMustDefine { name: String, item_ty: String },
    #[error("External {item_ty} item `{name}` must not have a definition. Maybe you meant to put it outside the external?")]
    ExternItemNoDefine { name: String, item_ty: String },
    #[error("Invalid type `{name}`. Only external sonance_builtin types are supported.")]
    NonBuiltinTy { name: String },
    #[error("Externals must not be defined inside other externals.")]
    NestedExternals,
    #[error("Link attribute for {item_ty} item `{name}` has no name parameter. Either add one or remove the link attribute.")]
    LinkAttributeWithoutName { name: String, item_ty: String },
    #[error("Unknown external protocol {protocol}.")]
    UnknownExternalProtocol { protocol: String },
}
