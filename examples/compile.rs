use shade_runner as sr;
use std::path::PathBuf;
use shaderc::ShaderKind;

fn main() {
    let project_root = std::env::current_dir().expect("failed to get root directory");

    // Compile a vertex shader
    let mut vert_path = project_root.clone();
    vert_path.push(PathBuf::from("examples/shaders/vert.glsl"));

    let vertex_shader = sr::load(vert_path, None, ShaderKind::Vertex, None)
        .expect("Failed to compile");


    // Compile a fragment shader
    let mut frag_path = project_root.clone();
    frag_path.push(PathBuf::from("examples/shaders/frag.glsl"));

    let fragment_shader = sr::load(frag_path, None, ShaderKind::Fragment, None)
        .expect("Failed to compile");

    let vertex_entry = sr::parse(&vertex_shader).expect("failed to parse");
    dbg!(vertex_entry);

    let fragment_entry = sr::parse(&fragment_shader).expect("failed to parse");
    dbg!(fragment_entry);
}
