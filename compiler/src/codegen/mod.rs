use {
    crate::parser::ParseResult,
    cranelift::{codegen::binemit::NullTrapSink, prelude::*},
    cranelift_module::{Linkage, Module},
    cranelift_object::{ObjectBackend, ObjectBuilder},
    thiserror::Error,
};

pub fn codegen(input: ParseResult) -> Result<Vec<u8>, CodegenError> {
    // Create a module using host configuation
    let mut module = {
        let isa =
            isa::lookup(target_lexicon::HOST)?.finish(settings::Flags::new(settings::builder()));
        let builder =
            ObjectBuilder::new(isa, "sonance", cranelift_module::default_libcall_names())?;
        Module::<ObjectBackend>::new(builder)
    };

    // Context objects
    let mut context = module.make_context();
    let mut builder_context = FunctionBuilderContext::new();

    // Create a function signature and attach it
    let mut signature = module.make_signature();
    signature.returns.push(AbiParam::new(types::I32));
    let id = module.declare_function(&input.name, Linkage::Export, &signature)?;
    context.func.signature = signature;

    // Build the function
    {
        let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);

        // Setup the EBB
        let block = builder.create_block();
        builder.append_block_params_for_function_params(block);
        builder.switch_to_block(block);
        builder.seal_block(block);

        // Fill in a const and ret instruction
        let result = builder.ins().iconst(types::I32, input.body as i64);
        builder.ins().return_(&[result]);

        // Finialize the EBB
        builder.seal_all_blocks();
        builder.finalize();
    }

    // Finialize the function
    module.define_function(id, &mut context, &mut NullTrapSink {})?;
    module.clear_context(&mut context);
    module.finalize_definitions();

    // Return bytecode
    Ok(module.finish().emit()?)
}

#[derive(Debug, Error)]
pub enum CodegenError {
    #[error("Failed to lookup instruction set")]
    Lookup(#[from] cranelift::codegen::isa::LookupError),
    #[error("Error while using cranelift Module")]
    Module(#[from] cranelift_module::ModuleError),
    #[error("Error while emitting object blob")]
    Object(#[from] cranelift_object::object::write::Error),
}
