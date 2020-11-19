use cranelift::codegen::Context;
use cranelift_module::FuncId;

use {
    crate::semantic,
    cranelift::{codegen::binemit::NullTrapSink, prelude::*},
    cranelift_module::Linkage,
    cranelift_object::ObjectBuilder,
    thiserror::Error,
};

type Module = cranelift_module::Module<cranelift_object::ObjectBackend>;

impl semantic::File {
    pub fn visit_codegen(self) -> Result<Vec<u8>, CodegenError> {
        // Create a module using host configuration
        let isa =
            isa::lookup(target_lexicon::HOST)?.finish(settings::Flags::new(settings::builder()));
        let builder =
            ObjectBuilder::new(isa, "sonance", cranelift_module::default_libcall_names())?;
        let mut module = Module::new(builder);

        for item in self.items {
            item.visit_codegen(&mut module)?;
        }

        module.finalize_definitions();

        // Return bytecode
        Ok(module.finish().emit()?)
    }
}

impl semantic::Item {
    pub fn visit_codegen(self, module: &mut Module) -> Result<(), CodegenError> {
        match self {
            Self::Function(func) => func.visit_codegen(module)?,
        }

        Ok(())
    }
}

impl semantic::FunctionSignature {
    pub fn visit_codegen(
        self,
        module: &mut Module,
        context: &mut Context,
    ) -> Result<FuncId, CodegenError> {
        context
            .func
            .signature
            .returns
            .push(AbiParam::new(types::I32));

        let id = module.declare_function(&self.name, Linkage::Export, &context.func.signature)?;
        Ok(id)
    }
}

impl semantic::Function {
    pub fn visit_codegen(self, module: &mut Module) -> Result<(), CodegenError> {
        let mut context = module.make_context();
        let mut builder_context = FunctionBuilderContext::new();

        // Setup signature and function builder
        let id = self.signature.visit_codegen(module, &mut context)?;
        let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);

        // Setup the EBB
        let block = builder.create_block();
        builder.append_block_params_for_function_params(block);
        builder.switch_to_block(block);
        builder.seal_block(block);

        // Fill in a const and ret instruction
        let result = self.body.visit_codegen(&mut builder)?;
        builder.ins().return_(&[result]);

        // Finalize the EBB
        builder.seal_all_blocks();
        builder.finalize();

        // Finalize the function
        module.define_function(id, &mut context, &mut NullTrapSink {})?;

        Ok(())
    }
}

impl semantic::Expression {
    pub fn visit_codegen(self, builder: &mut FunctionBuilder) -> Result<Value, CodegenError> {
        Ok(match self {
            Self::Literal(num) => builder.ins().iconst(types::I32, num as i64),
        })
    }
}

pub fn codegen(input: semantic::File) -> Result<Vec<u8>, CodegenError> {
    input.visit_codegen()
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
