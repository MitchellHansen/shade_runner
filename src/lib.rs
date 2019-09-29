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

pub fn load<T>(input: T, shader_kind: ShaderKind) -> Result<CompiledShader, Error>
    where
        T: AsRef<Path>,
{
    Ok(CompiledShader { spriv: compiler::compile(input, shader_kind).map_err(Error::Compile)? })
}


/// Loads and compiles the vertex shader
pub fn load_vertex<T>(vertex: T) -> Result<CompiledShader, Error>
    where
        T: AsRef<Path>,
{
    let vertex = compiler::compile(vertex, ShaderKind::Vertex).map_err(Error::Compile)?;
    Ok(CompiledShader { spriv: vertex })
}

/// Loads and compiles the fragment shader
pub fn load_fragment<T>(fragment: T) -> Result<CompiledShader, Error>
    where
        T: AsRef<Path>,
{
    let fragment = compiler::compile(fragment, ShaderKind::Fragment).map_err(Error::Compile)?;
    Ok(CompiledShader { spriv: fragment })
}

/// Loads and compiles the geometry shader
pub fn load_geometry<T>(geometry: T) -> Result<CompiledShader, Error>
    where
        T: AsRef<Path>,
{
    let geometry = compiler::compile(geometry, ShaderKind::Geometry).map_err(Error::Compile)?;
    Ok(CompiledShader { spriv: geometry })
}

/// Loads and compiles the tessellation shader
pub fn load_tessellation_control<T>(tessellation_control: T) -> Result<CompiledShader, Error>
    where
        T: AsRef<Path>,
{
    let tess = compiler::compile(tessellation_control, ShaderKind::TessControl).map_err(Error::Compile)?;
    Ok(CompiledShader { spriv: tess })
}

/// Loads and compiles the tessellation shader
pub fn load_tessellation_evaluation<T>(tessellation_evaluation: T) -> Result<CompiledShader, Error>
    where
        T: AsRef<Path>,
{
    let tess = compiler::compile(tessellation_evaluation, ShaderKind::TessEvaluation).map_err(Error::Compile)?;
    Ok(CompiledShader { spriv: tess })
}

pub fn load_compute<T>(compute: T) -> Result<CompiledShader, Error>
    where
        T: AsRef<Path>,
{
    let options = CompileOptions::new().ok_or(CompileError::CreateCompiler).unwrap();
    load_compute_with_options(compute, options)
}

pub fn load_compute_with_options<T>(compute: T, options: CompileOptions) -> Result<CompiledShader, Error>
    where
        T: AsRef<Path>,
{
    let compute = compiler::compile_with_options(compute, ShaderKind::Compute, options).map_err(Error::Compile)?;
    Ok(CompiledShader {
        spriv: compute,
    })
}

pub fn parse_compute(code: &CompiledShader) -> Result<Entry, Error> {
    reflection::create_compute_entry(&code.spriv)
}

/// Parses the shaders and gives an entry point
pub fn parse(code: &CompiledShader) -> Result<Entry, Error> {
    reflection::create_entry(&code.spriv)
}
