use ligen::prelude::*;
use ligen::ir::{Attributes, Attribute, Identifier};
use ligen::generator::{Context, FileSet, FileGenerator, FFIGenerator, ObjectVisitor};
use ligen::generator::File;
use std::path::PathBuf;

/// CMake project generator.
#[derive(Debug, Clone)]
pub struct Generator {
    attributes: Attributes
}

impl ligen::Generator for Generator {
    fn new(_context: &Context, attributes: &Attributes) -> Self {
        let attributes = attributes.clone();
        Self { attributes }
    }
}

impl FileGenerator for Generator {
    fn generate_files(&self, context: &Context, file_set: &mut FileSet, _implementation: Option<&ObjectVisitor>) {
        let generator_version = env!("CARGO_PKG_VERSION");
        let project_name = &context.arguments.crate_name;

        let has_cpp = self
            .attributes
            .attributes
            .iter()
            .any(|attribute| *attribute == Attribute::Group(Identifier::new("cpp"), Default::default()));

        let content = if has_cpp {
            format!(
                include_str!("CMakeLists.txt.cpp"),
                generator_version = generator_version,
                project_name = project_name
            )
        } else {
            format!(
                include_str!("CMakeLists.txt.c"),
                generator_version = generator_version,
                project_name = project_name
            )
        };
        let file = File::new(PathBuf::from("CMakeLists.txt"), content);
        file_set.insert(file);
    }
}

impl FFIGenerator for Generator {
    fn generate_ffi(&self, _context: &Context, _implementation: Option<&ObjectVisitor>) -> TokenStream {
        Default::default()
    }
}