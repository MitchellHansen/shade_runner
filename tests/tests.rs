use color_backtrace;
use difference::{Changeset, Difference};
use shade_runner::*;
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use vulkano::descriptor::descriptor::*;
use vulkano::descriptor::pipeline_layout::{PipelineLayoutDesc, PipelineLayoutDescPcRange};
use vulkano::format::*;
use vulkano::pipeline::shader::ShaderInterfaceDefEntry;
use shaderc::ShaderKind;

fn setup() {
    color_backtrace::install();
}

fn difference(e: &str, t: &str) -> String {
    let diffs = Changeset::new(&e, &t, "");
    diffs
        .diffs
        .iter()
        .filter(|d| match d {
            Difference::Add(_) => true,
            Difference::Rem(_) => true,
            _ => false,
        })
        .map(|d| match d {
            Difference::Add(a) => format!("add: {}", a),
            Difference::Rem(a) => format!("remove: {}", a),
            _ => "".to_string(),
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn descriptor_layout<T>(desc: &T) -> String
    where
        T: PipelineLayoutDesc,
{
    let num_sets = desc.num_sets();
    let mut r = format!("{:?}", num_sets);
    for n in 0..num_sets {
        let num_bindings = desc.num_bindings_in_set(n);
        r = format!("{:?}{:?}", r, num_bindings);
        for b in num_bindings {
            r = format!("{:?}{:?}", r, desc.descriptor(n, b));
        }
    }
    let num_push_constants = desc.num_push_constants_ranges();
    r = format!("{:?}{:?}", r, num_push_constants);
    for i in 0..num_push_constants {
        r = format!("{:?}{:?}", r, desc.push_constants_range(i));
    }
    r
}

fn parse<T>(input: T, shader_kind: ShaderKind) -> shade_runner::Entry
    where
        T: AsRef<Path>,
{
    let project_root = std::env::current_dir().expect("failed to get root directory");
    let mut path = project_root.clone();
    path.push(PathBuf::from("tests/shaders/"));

    let mut shader_path = path.clone();
    shader_path.push(input);

    let shader = shade_runner::load(shader_path, None, shader_kind, None).expect("Failed to compile");

    shade_runner::parse(&shader).unwrap()
}

fn do_test<T>(a: &T, b: &T)
    where
        T: std::fmt::Debug,
{
    let a = format!("{:?}", a);
    let b = format!("{:?}", b);
    assert_eq!(&a, &b, "\n\nDifference: {}", difference(&a, &b));
}

#[test]
fn test_shade1() {
    setup();
    let frag_target = Entry {
        input: Some(Input { inputs: Vec::new() }),
        output: Some(Output {
            outputs: vec![ShaderInterfaceDefEntry {
                location: 0..1,
                format: Format::R32G32B32A32Sfloat,
                name: Some(Cow::Borrowed("f_color")),
            }],
        }),
        layout: Layout {
            layout_data: LayoutData {
                num_sets: 0,
                num_bindings: HashMap::new(),
                descriptions: HashMap::new(),
                num_constants: 0,
                pc_ranges: Vec::new(),
            },
        },
    };

    let vert_target = Entry {
        input: Some(Input {
            inputs: vec![ShaderInterfaceDefEntry {
                location: 0..1,
                format: Format::R32G32Sfloat,
                name: Some(Cow::Borrowed("position")),
            }],
        }),
        output: Some(Output {
            outputs: Vec::new(),
        }),
        layout: Layout {
            layout_data: LayoutData {
                num_sets: 0,
                num_bindings: HashMap::new(),
                descriptions: HashMap::new(),
                num_constants: 0,
                pc_ranges: Vec::new(),
            },
        },
    };

    let vert_entry = parse("vert1.glsl", ShaderKind::Vertex);
    let frag_entry = parse("frag1.glsl", ShaderKind::Fragment);
    do_test(&vert_entry, &vert_target);
    do_test(&frag_entry, &frag_target);
}

#[test]
fn test_shade2() {
    setup();
    let frag_target = Entry {
        input: Some(Input {
            inputs: vec![
                ShaderInterfaceDefEntry {
                    location: 0..1,
                    format: Format::R32G32B32A32Sfloat,
                    name: Some(Cow::Borrowed("cool")),
                },
                ShaderInterfaceDefEntry {
                    location: 1..2,
                    format: Format::R32G32Sfloat,
                    name: Some(Cow::Borrowed("yep")),
                },
                ShaderInterfaceDefEntry {
                    location: 2..3,
                    format: Format::R32Sfloat,
                    name: Some(Cow::Borrowed("monkey")),
                },
            ],
        }),
        output: Some(Output {
            outputs: vec![ShaderInterfaceDefEntry {
                location: 0..1,
                format: Format::R32G32B32A32Sfloat,
                name: Some(Cow::Borrowed("f_color")),
            }],
        }),
        layout: Layout {
            layout_data: LayoutData {
                num_sets: 0,
                num_bindings: HashMap::new(),
                descriptions: HashMap::new(),
                num_constants: 0,
                pc_ranges: Vec::new(),
            },
        },
    };

    let vert_target = Entry {
        input: Some(Input {
            inputs: vec![ShaderInterfaceDefEntry {
                location: 0..1,
                format: Format::R32G32Sfloat,
                name: Some(Cow::Borrowed("position")),
            }],
        }),
        output: Some(Output {
            outputs: vec![
                ShaderInterfaceDefEntry {
                    location: 0..1,
                    format: Format::R32G32B32A32Sfloat,
                    name: Some(Cow::Borrowed("cool")),
                },
                ShaderInterfaceDefEntry {
                    location: 1..2,
                    format: Format::R32G32Sfloat,
                    name: Some(Cow::Borrowed("yep")),
                },
                ShaderInterfaceDefEntry {
                    location: 2..3,
                    format: Format::R32Sfloat,
                    name: Some(Cow::Borrowed("monkey")),
                },
            ],
        }),
        layout: Layout {
            layout_data: LayoutData {
                num_sets: 0,
                num_bindings: HashMap::new(),
                descriptions: HashMap::new(),
                num_constants: 0,
                pc_ranges: Vec::new(),
            },
        },
    };

    let vert_entry = parse("vert2.glsl", ShaderKind::Vertex);
    let frag_entry = parse("frag2.glsl", ShaderKind::Fragment);
    do_test(&vert_entry, &vert_target);
    do_test(&frag_entry, &frag_target);
}

#[test]
fn test_shade3() {
    setup();
    let frag_target = Entry {
        input: Some(Input { inputs: Vec::new() }),
        output: Some(Output {
            outputs: vec![ShaderInterfaceDefEntry {
                location: 0..1,
                format: Format::R32G32B32A32Sfloat,
                name: Some(Cow::Borrowed("f_color")),
            }],
        }),
        layout: Layout {
            layout_data: LayoutData {
                num_sets: 1,
                num_bindings: vec![(0, 1)].into_iter().collect(),
                descriptions: vec![(
                    0,
                    vec![(
                        0,
                        DescriptorDesc {
                            ty: DescriptorDescTy::CombinedImageSampler(DescriptorImageDesc {
                                sampled: true,
                                dimensions: DescriptorImageDescDimensions::TwoDimensional,
                                format: None,
                                multisampled: false,
                                array_layers: DescriptorImageDescArray::NonArrayed,
                            }),
                            array_count: 1,
                            stages: ShaderStages {
                                fragment: true,
                                ..ShaderStages::none()
                            },
                            readonly: true,
                        },
                    )]
                        .into_iter()
                        .collect(),
                )]
                    .into_iter()
                    .collect(),
                num_constants: 0,
                pc_ranges: Vec::new(),
            },
        },
    };
    let vert_target = Entry {
        input: Some(Input {
            inputs: vec![ShaderInterfaceDefEntry {
                location: 0..1,
                format: Format::R32G32Sfloat,
                name: Some(Cow::Borrowed("position")),
            }],
        }),
        output: Some(Output {
            outputs: Vec::new(),
        }),
        layout: Layout {
            layout_data: LayoutData {
                num_sets: 0,
                num_bindings: HashMap::new(),
                descriptions: HashMap::new(),
                num_constants: 0,
                pc_ranges: Vec::new(),
            },
        },
    };

    let vert_entry = parse("vert3.glsl", ShaderKind::Vertex);
    let frag_entry = parse("frag3.glsl", ShaderKind::Fragment);

    do_test(&vert_entry.input, &vert_target.input);
    do_test(&vert_entry.output, &vert_target.output);

    do_test(&frag_entry.input, &frag_target.input);
    do_test(&frag_entry.output, &frag_target.output);

    do_test(
        &descriptor_layout(&frag_entry.layout),
        &descriptor_layout(&frag_target.layout),
    );
    do_test(
        &descriptor_layout(&vert_entry.layout),
        &descriptor_layout(&vert_target.layout),
    );
}

#[test]
fn test_shade4() {
    setup();
    let frag_target = Entry {
        input: Some(Input { inputs: Vec::new() }),
        output: Some(Output {
            outputs: vec![ShaderInterfaceDefEntry {
                location: 0..1,
                format: Format::R32G32B32A32Sfloat,
                name: Some(Cow::Borrowed("f_color")),
            }],
        }),
        layout: Layout {
            layout_data: LayoutData {
                num_sets: 0,
                num_bindings: HashMap::new(),
                descriptions: HashMap::new(),
                num_constants: 1,
                pc_ranges: vec![PipelineLayoutDescPcRange {
                    offset: 0,
                    size: 16,
                    stages: ShaderStages {
                        fragment: true,
                        ..ShaderStages::none()
                    },
                }],
            },
        },
    };
    let vert_target = Entry {
        input: Some(Input {
            inputs: vec![ShaderInterfaceDefEntry {
                location: 0..1,
                format: Format::R32G32Sfloat,
                name: Some(Cow::Borrowed("position")),
            }],
        }),
        output: Some(Output {
            outputs: Vec::new(),
        }),
        layout: Layout {
            layout_data: LayoutData {
                num_sets: 0,
                num_bindings: HashMap::new(),
                descriptions: HashMap::new(),
                num_constants: 0,
                pc_ranges: Vec::new(),
            },
        },
    };

    let vert_entry = parse("vert4.glsl", ShaderKind::Vertex);
    let frag_entry = parse("frag4.glsl", ShaderKind::Fragment);

    do_test(&vert_entry.input, &vert_target.input);
    do_test(&vert_entry.output, &vert_target.output);

    do_test(&frag_entry.input, &frag_target.input);
    do_test(&frag_entry.output, &frag_target.output);

    do_test(
        &descriptor_layout(&frag_entry.layout),
        &descriptor_layout(&frag_target.layout),
    );
    do_test(
        &descriptor_layout(&vert_entry.layout),
        &descriptor_layout(&vert_target.layout),
    );
}
