use thiserror::Error;

pub type Result<T, E = ItemsError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum ItemsError {
    #[error("Local {item_ty} item `{name}` must have a definition. Maybe you meant to put it inside an external?")]
    LocalItemMustDefine { name: String, item_ty: String },
    #[error("External {item_ty} item `{name}` must not have a definition. Maybe you meant to put it outside the external?")]
    ExternItemNoDefine { name: String, item_ty: String },
    #[error("Invalid type `{name}`. Only external sonance_builtin types are supported.")]
    NonBuiltinTy { name: String },
    #[error("Tried to link to unknown builtin type `{name}`.")]
    UnknownBuiltinTy { name: String },
    #[error("Unknown type `{ty_name}` in signature of {item_ty} item `{item_name}`")]
    UnknownTy {
        ty_name: String,
        item_name: String,
        item_ty: String,
    },
    #[error("Externals must not be defined inside other externals.")]
    NestedExternals,
    #[error("Link attribute for {item_ty} item `{name}` has no name parameter. Either add one or remove the link attribute.")]
    LinkAttributeWithoutName { name: String, item_ty: String },
    #[error("Unknown external protocol {protocol}.")]
    UnknownExternalProtocol { protocol: String },
}
