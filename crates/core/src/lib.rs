use ligen::prelude::*;
use ligen::ir::Attributes;
use ligen::generator::{Context, FileSet, FileGenerator, FFIGenerator, ImplementationVisitor};
use ligen::generator::File;
use std::path::PathBuf;

/// CMake project generator.
#[derive(Debug, Clone)]
pub struct Generator;

impl ligen::Generator for Generator {
    fn new(_context: &Context, _attributes: &Attributes) -> Self {
        Self
    }
}

impl FileGenerator for Generator {
    fn generate_files(&self, context: &Context, file_set: &mut FileSet, _implementation: Option<&ImplementationVisitor>) {
        let generator_version = env!("CARGO_PKG_VERSION");
        let project_name = &context.arguments.crate_name;

        let content = format!(
            include_str!("CMakeLists.txt"),
            generator_version = generator_version,
            project_name = project_name
        );
        let file = File::new(PathBuf::from("CMakeLists.txt"), content);
        file_set.insert(file);
    }
}

impl FFIGenerator for Generator {
    fn generate_ffi(&self, _context: &Context, _implementation: Option<&ImplementationVisitor>) -> TokenStream {
        Default::default()
    }
}