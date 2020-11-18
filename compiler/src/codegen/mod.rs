use {
    crate::parser::ParseResult,
    cranelift::{codegen::binemit::NullTrapSink, prelude::*},
    cranelift_module::{Linkage, Module},
    cranelift_object::{ObjectBackend, ObjectBuilder},
    thiserror::Error,
};

pub fn codegen(input: ParseResult) -> Result<Vec<u8>, BackendError> {
    let mut module = {
        let isa =
            isa::lookup(target_lexicon::HOST)?.finish(settings::Flags::new(settings::builder()));
        let builder =
            ObjectBuilder::new(isa, "sonance", cranelift_module::default_libcall_names())?;
        Module::<ObjectBackend>::new(builder)
    };

    let mut ctx = module.make_context();
    let mut builder_context = FunctionBuilderContext::new();

    let mut signature = module.make_signature();
    signature.returns.push(AbiParam::new(types::I32));
    let id = module.declare_function(&input.name, Linkage::Export, &signature)?;
    ctx.func.signature = signature;

    {
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_context);

        let block = builder.create_block();
        builder.append_block_params_for_function_params(block);
        builder.switch_to_block(block);
        builder.seal_block(block);

        let result = builder.ins().iconst(types::I32, input.body as i64);
        builder.ins().return_(&[result]);

        builder.seal_all_blocks();
        builder.finalize();
    }

    module.define_function(id, &mut ctx, &mut NullTrapSink {})?;
    module.clear_context(&mut ctx);
    module.finalize_definitions();

    Ok(module.finish().emit()?)
}

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("Failed to lookup instruction set")]
    Lookup(#[from] cranelift::codegen::isa::LookupError),
    #[error("Error while using cranelift Module")]
    Module(#[from] cranelift_module::ModuleError),
    #[error("Error while emitting object blob")]
    Object(#[from] cranelift_object::object::write::Error),
}
