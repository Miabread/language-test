use {
    cranelift::{codegen::binemit::NullTrapSink, prelude::*},
    cranelift_module::{Linkage, Module},
    cranelift_object::{ObjectBuilder, ObjectModule},
    thiserror::Error,
};

pub struct File<'a> {
    pub name: &'a str,
    pub number: u8,
}

pub fn compile(input: &File) -> Result<Vec<u8>, CodegenError> {
    // Create a module using host configuration
    let isa = isa::lookup(target_lexicon::HOST)?.finish(settings::Flags::new(settings::builder()));
    let builder = ObjectBuilder::new(isa, "sonance", cranelift_module::default_libcall_names())?;
    let mut module = ObjectModule::new(builder);

    let mut context = module.make_context();
    let mut builder_context = FunctionBuilderContext::new();

    // Setup signature and function builder
    context
        .func
        .signature
        .returns
        .push(AbiParam::new(types::I8));
    let id = module.declare_function(&input.name, Linkage::Export, &context.func.signature)?;
    let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);

    // Setup the EBB
    let block = builder.create_block();
    builder.append_block_params_for_function_params(block);
    builder.switch_to_block(block);
    builder.seal_block(block);

    // Fill in a const and ret instruction
    let result = builder.ins().iconst(types::I8, input.number as i64);
    builder.ins().return_(&[result]);

    // Finalize the EBB
    builder.seal_all_blocks();
    builder.finalize();

    // Finalize the function
    module.define_function(id, &mut context, &mut NullTrapSink {})?;

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
