pub mod error;
pub mod func;

use crate::p2_items as items;
use crate::p3_body as body;
use cranelift::{codegen::binemit::NullTrapSink, prelude::*};
use cranelift_module::{FuncId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use std::collections::HashMap;

pub use error::*;

pub fn compile(input: body::Program) -> Result<Vec<u8>> {
    let mut context = CodegenContext::new(input.items)?;

    for (name, items::TyItem::Builtin(ty)) in &context.items.tys {
        context.tys.insert(
            name.clone(),
            match ty {
                items::BuiltinTy::Integer32 => types::I32,
                items::BuiltinTy::Float32 => types::F32,
            },
        );
    }

    visit_func_items(&mut context)?;

    visit_func_bodies(&input.func_bodies, &mut context)?;

    Ok(context.module.finish().emit()?)
}

fn visit_func_bodies(bodies: &[body::FuncBody], context: &mut CodegenContext) -> Result<()> {
    let mut ctx = context.module.make_context();
    let mut builder_context = FunctionBuilderContext::new();

    for func in bodies {
        ctx.func.signature = context.funcs[&func.signature.0].signature.clone();

        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_context);

        let block = builder.create_block();

        builder.append_block_params_for_function_params(block);
        builder.switch_to_block(block);
        builder.seal_block(block);

        context.module.clear_context(&mut ctx);
    }

    Ok(())
}

fn visit_func_items(context: &mut CodegenContext) -> Result<()> {
    for (name, func) in &context.items.funcs {
        match func {
            items::FuncItem::Local(func) => {
                let mut signature = context.module.make_signature();
                signature
                    .returns
                    .push(AbiParam::new(context.tys[&func.return_ty.0]));

                for param in &func.parameters {
                    signature.params.push(AbiParam::new(context.tys[&param.0]));
                }

                let id = context
                    .module
                    .declare_function(&name, Linkage::Export, &signature)?;

                context
                    .funcs
                    .insert(name.to_owned(), BackendFunc { id, signature });
            }
            items::FuncItem::External(func) => {
                let mut signature = context.module.make_signature();
                signature
                    .returns
                    .push(AbiParam::new(context.tys[&func.return_ty.0]));

                for param in &func.parameters {
                    signature.params.push(AbiParam::new(context.tys[&param.0]));
                }

                let id = context
                    .module
                    .declare_function(&name, Linkage::Import, &signature)?;

                context
                    .funcs
                    .insert(name.to_owned(), BackendFunc { id, signature });
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct BackendFunc {
    id: FuncId,
    signature: Signature,
}

struct CodegenContext {
    items: items::Program,
    tys: HashMap<String, Type>,
    funcs: HashMap<String, BackendFunc>,
    module: ObjectModule,
}

impl CodegenContext {
    fn new(items: items::Program) -> Result<Self> {
        let isa =
            isa::lookup(target_lexicon::HOST)?.finish(settings::Flags::new(settings::builder()));
        let builder =
            ObjectBuilder::new(isa, "sonance", cranelift_module::default_libcall_names())?;

        Ok(CodegenContext {
            items,
            module: ObjectModule::new(builder),
            funcs: HashMap::new(),
            tys: HashMap::new(),
        })
    }
}
