use crate::error::Error;
use crate::layouts::*;
use crate::{sr, CompiledShader};
use crate::srvk::{DescriptorDescInfo, SpirvTy};
use crate::vk::descriptor::descriptor::*;
use crate::vk::descriptor::pipeline_layout::PipelineLayoutDescPcRange;
use crate::vk::pipeline::shader::ShaderInterfaceDefEntry;
use crate::CompiledShaders;
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::TryFrom;

pub struct ShaderInterfaces {
    pub inputs: Vec<ShaderInterfaceDefEntry>,
    pub outputs: Vec<ShaderInterfaceDefEntry>,
}

#[derive(Debug, Clone, Default)]
pub struct LayoutData {
    pub num_sets: usize,
    pub num_bindings: HashMap<usize, usize>,
    pub descriptions: HashMap<usize, HashMap<usize, DescriptorDesc>>,
    pub num_constants: usize,
    pub pc_ranges: Vec<PipelineLayoutDescPcRange>,
}

pub fn create_entry(spirv: &Vec<u32>) -> Result<Entry, Error> {

    let vertex_interfaces = create_interfaces(spirv)?;
    let vertex_layout = create_layouts(spirv)?;

    let input = Some(Input {
        inputs: vertex_interfaces.inputs,
    });
    let output = Some(Output {
        outputs: vertex_interfaces.outputs,
    });
    let layout = Layout {
        layout_data: vertex_layout,
    };

    Ok(Entry {
        input,
        output,
        layout,
    })
}

pub fn create_compute_entry(spirv: &Vec<u32>) -> Result<Entry, Error> {

    let compute_layout = create_layouts(spirv)?;

    let layout = Layout {
        layout_data: compute_layout,
    };

    Ok(Entry {
        input: None,
        output: None,
        layout,
    })
}

fn create_interfaces(data: &[u32]) -> Result<ShaderInterfaces, Error> {
    sr::ShaderModule::load_u32_data(data)
        .map_err(|e| Error::LoadingData(e.to_string()))
        .map(|m| {
            let inputs = m
                .enumerate_input_variables(None)
                .map_err(|e| Error::LoadingData(e.to_string()))
                .and_then(|inputs| {
                    inputs
                        .iter()
                        .filter(|i| {
                            !i.decoration_flags
                                .contains(sr::types::ReflectDecorationFlags::BUILT_IN)
                        })
                        .map(|i| Ok(ShaderInterfaceDefEntry {
                            location: i.location..(i.location + 1),
                            format: SpirvTy::try_from(i.format)?.inner(),
                            name: Some(Cow::from(i.name.clone())),
                        }))
                        .collect::<Result<Vec<ShaderInterfaceDefEntry>, _>>()
                });
            let outputs = m
                .enumerate_output_variables(None)
                .map_err(|e| Error::LoadingData(e.to_string()))
                .and_then(|outputs| {
                    outputs
                        .iter()
                        .filter(|i| {
                            !i.decoration_flags
                                .contains(sr::types::ReflectDecorationFlags::BUILT_IN)
                        })
                        .map(|i| Ok(ShaderInterfaceDefEntry {
                            location: i.location..(i.location + 1),
                            format: SpirvTy::try_from(i.format)?.inner(),
                            name: Some(Cow::from(i.name.clone())),
                        }))
                        .collect::<Result<Vec<ShaderInterfaceDefEntry>, _>>()
                });
            inputs.and_then(|inputs| outputs.map(|outputs| ShaderInterfaces { inputs, outputs } ))
        })
    .and_then(|t| t)
}

fn create_layouts(data: &[u32]) -> Result<LayoutData, Error> {
    let mut ret = sr::ShaderModule::load_u32_data(data);

    ret.map(|m| {
            let descs: Result<_, Error> = m
                .enumerate_descriptor_sets(None)
                .map_err(|e| Error::LoadingData(e.to_string()))
                .and_then(|sets| {
                    let num_sets = sets.len();
                    let num_bindings = sets
                        .iter()
                        .map(|i| (i.set as usize, i.bindings.len()))
                        .collect::<HashMap<usize, usize>>();
                    let descriptions = sets
                        .iter()
                        .map(|i| {
                            let desc = i
                                .bindings
                                .iter()
                                .map(|b| {
                                    let info = DescriptorDescInfo {
                                        descriptor_type: b.descriptor_type,
                                        image: b.image,
                                    };
                                    let ty = SpirvTy::<DescriptorDescTy>::try_from(info)?.inner();
                                    let stages = ShaderStages::none();
                                    let d = DescriptorDesc {
                                        ty,
                                        array_count: b.count,
                                        stages,
                                        // TODO this is what vulkan_shaders does but I don't think
                                        // it's correct
                                        readonly: true,
                                    };
                                    Ok((b.binding as usize, d))
                                })
                                .collect::<Result<HashMap<usize, DescriptorDesc>, Error>>();
                            desc.and_then(|d| Ok((i.set as usize, d)))
                        })
                        .collect::<Result<HashMap<usize, _>, Error>>();
                    descriptions.map(|d| (num_sets, num_bindings, d))
                });
            let pcs = m
                .enumerate_push_constant_blocks(None)
                .map_err(|e| Error::LoadingData(e.to_string()))
                .map(|constants| {
                    let num_constants = constants.len();
                    let pc_ranges = constants
                        .iter()
                        .map(|pc| PipelineLayoutDescPcRange {
                            offset: pc.offset as usize,
                            size: pc.size as usize,
                            stages: ShaderStages::all(),
                        })
                        .collect::<Vec<PipelineLayoutDescPcRange>>();
                    (num_constants, pc_ranges)
                });
            descs.and_then(|(num_sets, num_bindings, descriptions)| {
                pcs.map(|(num_constants, pc_ranges)| LayoutData {
                    num_sets,
                    num_bindings,
                    descriptions,
                    num_constants,
                    pc_ranges,
                })
            })
        })
        .map_err(|e| Error::LoadingData(e.to_string()))
        .and_then(|t| t)
}
