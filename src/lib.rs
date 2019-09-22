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

/// Loads and compiles the vertex shader
pub fn load_vertex<T>(vertex: T) -> Result<CompiledShader, Error>
    where
        T: AsRef<Path>,
{
    let vertex = compiler::compile(vertex, ShaderKind::Vertex).map_err(Error::Compile)?;
    Ok(CompiledShader{ spriv: vertex })
}

/// Loads and compiles the fragment shader
pub fn load_fragment<T>(fragment: T) -> Result<CompiledShader, Error>
    where
        T: AsRef<Path>,
{
    let fragment = compiler::compile(vertex, ShaderKind::Fragment).map_err(Error::Compile)?;
    Ok(CompiledShader{ spriv: fragment })
}

/// Loads and compiles the geometry shader
pub fn load_geometry<T>(geometry: T) -> Result<CompiledShader, Error>
    where
        T: AsRef<Path>,
{
    let geometry = compiler::compile(vertex, ShaderKind::Geometry).map_err(Error::Compile)?;
    Ok(CompiledShader{ spriv: geometry })
}

/// Loads and compiles the tessellation shader
pub fn load_tessellation_control<T>(geometry: T) -> Result<CompiledShader, Error>
    where
        T: AsRef<Path>,
{
    let tess = compiler::compile(vertex, ShaderKind::TessControl).map_err(Error::Compile)?;
    Ok(CompiledShader{ spriv: tess })
}

/// Loads and compiles the tessellation shader
pub fn load_tessellation_evaluation<T>(geometry: T) -> Result<CompiledShader, Error>
    where
        T: AsRef<Path>,
{
    let tess = compiler::compile(vertex, ShaderKind::TessEvaluation).map_err(Error::Compile)?;
    Ok(CompiledShader{ spriv: tess })
}

// TODO this should be incorpoarted into load but that would be
// a breaking change. Do this in next major version
pub fn load_compute<T>(compute: T) -> Result<CompiledShaders, Error>
where
    T: AsRef<Path>,
{
    let options = CompileOptions::new().ok_or(CompileError::CreateCompiler).unwrap();
    load_compute_with_options(compute, options)
}

pub fn load_compute_with_options<T>(compute: T, options: CompileOptions) -> Result<CompiledShaders, Error>
    where
        T: AsRef<Path>,
{
    let compute = compiler::compile_with_options(compute, ShaderKind::Compute, options).map_err(Error::Compile)?;
    Ok(CompiledShaders{
        vertex: Vec::new(),
        fragment: Vec::new(),
        compute,
    })
}

pub fn parse_compute(code: &CompiledShaders) -> Result<Entry, Error> {
    reflection::create_compute_entry(code)
}

/// Parses the shaders and gives an entry point
pub fn parse(code: &CompiledShaders) -> Result<Entry, Error> {
    reflection::create_entry(code)
}
