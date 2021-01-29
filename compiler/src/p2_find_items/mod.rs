pub mod data;
pub mod error;

use {crate::p1_syntax as syntax, std::collections::HashMap};
pub use {data::*, error::*};

pub fn find_items(input: syntax::File) -> Result<Program> {
    let mut tys = HashMap::new();
    let mut funcs = HashMap::new();

    for item in input.items {
        match item {
            syntax::Item::Ty(ty) => return Err(FindItemsError::NonBuiltinTy { name: ty.name }),
            syntax::Item::Func(func) => {
                if let Some(body) = func.body {
                    funcs.insert(
                        func.name,
                        RawFuncPrototype {
                            parameters: func.parameters.into_iter().map(|p| p.ty).collect(),
                            return_ty: func.return_ty,
                            linkage: RawFuncLinkage::Local(body),
                        },
                    );
                } else {
                    return Err(FindItemsError::LocalItemMustDefine {
                        name: func.name,
                        item_ty: "func".to_owned(),
                    });
                }
            }
            syntax::Item::External(external) => {
                let abi = match &external.protocol[..] {
                    "sonance_builtin" => ExternalProtocol::sonance_builtin,
                    "c" => ExternalProtocol::c,
                    _ => {
                        return Err(FindItemsError::UnknownExternalProtocol {
                            protocol: external.protocol,
                        });
                    }
                };

                for item in external.items {
                    match item {
                        syntax::Item::External(_) => return Err(FindItemsError::NestedExternals),
                        syntax::Item::Ty(ty) => {
                            if abi != ExternalProtocol::sonance_builtin {
                                return Err(FindItemsError::NonBuiltinTy { name: ty.name });
                            }

                            let linkage = get_link_name(&ty.name, "type", &ty.attributes)?;

                            tys.insert(ty.name, RawTyPrototype { linkage });
                        }
                        syntax::Item::Func(func) => {
                            if func.body.is_some() {
                                return Err(FindItemsError::ExternItemNoDefine {
                                    name: func.name,
                                    item_ty: "func".to_owned(),
                                });
                            }

                            let linkage = get_link_name(&func.name, "func", &func.attributes)?;
                            let linkage = match abi {
                                ExternalProtocol::sonance_builtin => {
                                    RawFuncLinkage::Builtin(linkage)
                                }
                                ExternalProtocol::c => RawFuncLinkage::Foreign(linkage),
                            };

                            funcs.insert(
                                func.name,
                                RawFuncPrototype {
                                    linkage,
                                    parameters: func.parameters.into_iter().map(|p| p.ty).collect(),
                                    return_ty: func.return_ty,
                                },
                            );
                        }
                    }
                }
            }
        }
    }

    Ok(Program { funcs, tys })
}

fn get_link_name(name: &str, item_ty: &str, attributes: &[syntax::Attribute]) -> Result<String> {
    if let Some(link) = attributes.iter().find(|a| a.name == "link") {
        link.arguments
            .get(0)
            .cloned()
            .ok_or_else(|| FindItemsError::LinkAttributeWithoutName {
                name: name.to_owned(),
                item_ty: item_ty.to_owned(),
            })
    } else {
        Ok(name.to_owned())
    }
}
