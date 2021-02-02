pub mod data;
pub mod error;

use crate::p1_syntax as syntax;
pub use {data::*, error::*};

pub fn check_items(input: syntax::File) -> Result<Program> {
    let mut program = Program::default();

    for item in input.items {
        match item {
            syntax::Item::External(external) => visit_external(external, &mut program)?,
            syntax::Item::Ty(ty) => return Err(ItemsError::NonBuiltinTy { name: ty.name }),
            syntax::Item::Func(func) => visit_func(func, &mut program)?,
        }
    }

    Ok(program)
}

fn get_protocol(protocol: &str) -> Result<Protocol, ItemsError> {
    Ok(match protocol {
        "sonance_builtin" => Protocol::sonance_builtin,
        "c" => Protocol::c,
        _ => {
            return Err(ItemsError::UnknownExternalProtocol {
                protocol: protocol.to_owned(),
            });
        }
    })
}

fn visit_external(external: syntax::ExternalItem, program: &mut Program) -> Result<(), ItemsError> {
    let protocol = get_protocol(&external.protocol)?;

    for item in external.items {
        match item {
            syntax::Item::External(_) => return Err(ItemsError::NestedExternals),
            syntax::Item::Ty(ty) => visit_external_ty(ty, protocol, program)?,
            syntax::Item::Func(func) => visit_external_func(func, protocol, program)?,
        }
    }

    Ok(())
}

fn visit_external_ty(
    ty: syntax::TyItem,
    protocol: Protocol,
    program: &mut Program,
) -> Result<(), ItemsError> {
    let name = ty.name;

    if protocol != Protocol::sonance_builtin {
        return Err(ItemsError::NonBuiltinTy { name });
    }

    use BuiltinTy::*;
    let ty = match get_link_name(&name, "type", &ty.attributes)?.as_str() {
        "Integer32" => Integer32,
        "Float32" => Float32,
        _ => return Err(ItemsError::UnknownBuiltinTy { name }),
    };

    program.tys.insert(name, TyItem::Builtin(ty));

    Ok(())
}

fn visit_external_func(
    func: syntax::FuncItem,
    protocol: Protocol,
    program: &mut Program,
) -> Result<(), ItemsError> {
    fn get_type(ty_name: String, func_name: &str, program: &mut Program) -> Result<TyItemRef> {
        if !program.tys.contains_key(&ty_name) {
            return Err(ItemsError::UnknownTy {
                ty_name,
                item_name: func_name.to_owned(),
                item_ty: "func".to_owned(),
            });
        }

        Ok(TyItemRef(ty_name))
    }

    if func.body.is_some() {
        return Err(ItemsError::ExternItemNoDefine {
            name: func.name,
            item_ty: "func".to_owned(),
        });
    }

    let name = &func.name;
    let link_name = get_link_name(&func.name, "func", &func.attributes)?;

    let parameters = func
        .parameters
        .into_iter()
        .map(|p| get_type(p.ty, name, program))
        .collect::<Result<_>>()?;

    let return_ty = get_type(func.return_ty, &func.name, program)?;

    program.funcs.insert(
        func.name,
        FuncItem::External(ExternalFuncItem {
            protocol,
            link_name,
            parameters,
            return_ty,
        }),
    );

    Ok(())
}

fn visit_func(func: syntax::FuncItem, program: &mut Program) -> Result<(), ItemsError> {
    fn get_type(ty_name: String, func_name: &str, program: &mut Program) -> Result<TyItemRef> {
        if !program.tys.contains_key(&ty_name) {
            return Err(ItemsError::UnknownTy {
                ty_name,
                item_name: func_name.to_owned(),
                item_ty: "func".to_owned(),
            });
        }

        Ok(TyItemRef(ty_name))
    }

    let name = func.name;
    let body = func.body.ok_or_else(|| ItemsError::LocalItemMustDefine {
        name: name.clone(),
        item_ty: "func".to_owned(),
    })?;

    let parameters = func
        .parameters
        .into_iter()
        .map(|p| get_type(p.ty, &name, program))
        .collect::<Result<_>>()?;

    let return_ty = get_type(func.return_ty, &name, program)?;

    program.funcs.insert(
        name.clone(),
        FuncItem::Local(LocalFuncItem {
            link_name: name,
            parameters,
            return_ty,
            body,
        }),
    );

    Ok(())
}

fn get_link_name(name: &str, item_ty: &str, attributes: &[syntax::Attribute]) -> Result<String> {
    if let Some(link) = attributes.iter().find(|a| a.name == "link") {
        link.arguments
            .get(0)
            .cloned()
            .ok_or_else(|| ItemsError::LinkAttributeWithoutName {
                name: name.to_owned(),
                item_ty: item_ty.to_owned(),
            })
    } else {
        Ok(name.to_owned())
    }
}
