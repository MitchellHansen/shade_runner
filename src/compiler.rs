use crate::error::CompileError;
use shaderc::{IncludeType, ResolvedInclude};
use shaderc::{ShaderKind, CompileOptions};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::borrow::Cow;

pub fn compile<T>(path: T, include_path: Option<T>, shader_kind: ShaderKind, compiler_options: Option<CompileOptions>) -> Result<Vec<u32>, CompileError>
    where
        T: AsRef<Path>,
{
    compile_with_options(&read_to_string(&path), include_path, shader_kind, compiler_options)
}

pub fn compile_from_string<T>(input: &str, include_path: Option<T>, shader_kind: ShaderKind, compiler_options: Option<CompileOptions>) -> Result<Vec<u32>, CompileError>
    where
        T: AsRef<Path>,
{
    compile_with_options(input, include_path, shader_kind, compiler_options)
}

pub fn compile_with_options<T>(src: &str, include_path: Option<T>, shader_kind: ShaderKind, options: Option<CompileOptions>)
                               -> Result<Vec<u32>, CompileError>
    where
        T: AsRef<Path>,
{
    // TODO Probably shouldn't create this every time.
    let mut compiler = shaderc::Compiler::new().ok_or(CompileError::CreateCompiler)?;

    let mut options = {
        match options {
            None => CompileOptions::new().ok_or(CompileError::CreateCompiler).unwrap(),
            Some(option) => option,
        }
    };

    let path = {
        if let Some(path) = &include_path {
            options.set_include_callback(|path, include_type, folder_path, depth| {
                get_include(path, include_type, folder_path, depth)
            });
            path.as_ref().to_str().ok_or(CompileError::InvalidPath)?
        } else {
            options.set_include_callback(|path, include_type, folder_path, depth| {
                default_get_include(path, include_type, folder_path, depth)
            });
            ""
        }
    };

    let result = compiler
        .compile_into_spirv(
            src,
            shader_kind,
            path,
            "main",
            Some(&options),
        )
        .map_err(CompileError::Compile)?;
    let data = result.as_binary();
    Ok(data.to_owned())
}

pub fn read_to_string<'a, T>(path: &T) -> Cow<'a, str>
    where
        T: AsRef<Path>,
{
    let mut f = File::open(path).map_err(CompileError::Open).expect("");
    let mut src = String::new();
    f.read_to_string(&mut src).map_err(CompileError::Open).expect("");
    Cow::Owned(src)
}

fn default_get_include(
    path: &str,
    include_type: IncludeType,
    folder_path: &str,
    _depth: usize,
) -> Result<ResolvedInclude, String> {
    // TODO: Does this print out anything meaningful?
    Err(format!("No include path given for {}", path).to_string())
}

fn get_include(
    path: &str,
    include_type: IncludeType,
    folder_path: &str,
    _depth: usize,
) -> Result<ResolvedInclude, String> {
    match include_type {
        IncludeType::Relative => {
            let p = Path::new(path);
            let mut folder = PathBuf::from(folder_path);
            folder.pop();
            folder.push(p);
            let p = folder;
            if !p.is_file() {
                return Err("Include doesn't point to file".to_string());
            }

            let resolved_name = p
                .to_str()
                .ok_or("Path has invalid characters".to_string())?
                .to_owned();
            let p = p.canonicalize().map_err(|_| "Failed to parse include path".to_string())?;
            let mut content = String::new();
            File::open(p)
                .map_err(|_| "Couldn't open include directory".to_string())?
                .read_to_string(&mut content)
                .map_err(|_| "Failed to read included shader".to_string())?;
            Ok(ResolvedInclude {
                resolved_name,
                content,
            })
        }
        IncludeType::Standard => Err("Standard includes are unimplemented".to_string()),
    }
}
