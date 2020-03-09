mod compiler;
pub mod error;
mod reflection;
mod srvk;
pub mod layouts;
mod watch;

pub use layouts::*;
pub use reflection::LayoutData;
pub use watch::{Message, Watch};
pub use error::*;

use shaderc::CompileOptions;
use spirv_reflect as sr;
use vulkano as vk;
use std::path::Path;
use shaderc::ShaderKind;
use std::borrow::Borrow;

#[derive(Clone)]
pub struct CompiledShaders {
    pub vertex: Vec<u32>,
    pub fragment: Vec<u32>,
    pub compute: Vec<u32>,
}

#[derive(Clone)]
pub struct CompiledShader {
    pub spriv: Vec<u32>,
}

pub fn load<T>(input: T, include_path: Option<T>, shader_kind: ShaderKind, compiler_options: Option<CompileOptions>)
               -> Result<CompiledShader, Error>
    where
        T: AsRef<Path>,
{
    Ok(CompiledShader {
        spriv: compiler::compile(input, include_path, shader_kind, compiler_options).map_err(Error::Compile)?
    })
}

pub fn load_from_string<T>(source: &str, include_path: Option<T>, shader_kind: ShaderKind, compiler_options: Option<CompileOptions>) -> Result<CompiledShader, Error>
    where
        T: AsRef<Path>,
{
    Ok(CompiledShader {
        spriv: compiler::compile_from_string(source, include_path, shader_kind, compiler_options).map_err(Error::Compile)?
    })
}

pub fn load_compute<T>(compute: T, compiler_options: Option<CompileOptions>) -> Result<CompiledShader, Error>
    where
        T: AsRef<Path>,
{
    load_compute_from_string(&compiler::read_to_string(&compute), Some(compute), compiler_options)
}

pub fn load_compute_from_string<T>(source : &str, include_path : Option<T>, compiler_options: Option<CompileOptions>) -> Result<CompiledShader, Error>
    where
        T: AsRef<Path>,
{
    Ok(CompiledShader {
        spriv: compiler::compile_from_string(source, include_path, ShaderKind::Compute, compiler_options).map_err(Error::Compile)?
    })
}

pub fn parse_compute(code: &CompiledShader) -> Result<Entry, Error> {
    reflection::create_compute_entry(&code.spriv)
}

/// Parses the shaders and gives an entry point
pub fn parse(code: &CompiledShader) -> Result<Entry, Error> {
    reflection::create_entry(&code.spriv)
}
