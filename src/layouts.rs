use crate::vk;
use vk::pipeline::shader::*;
pub use vk::pipeline::shader::ShaderInterfaceDef;
use vk::descriptor::descriptor::*;
use vk::descriptor::pipeline_layout::*;
use crate::reflection::LayoutData;

#[derive(Debug, Clone, Default)]
pub struct Entry {
    pub input: Option<Input>,
    pub output: Option<Output>,
    pub layout: Layout,
}

#[derive(Debug, Clone, Default)]
pub struct Input {
    pub inputs: Vec<ShaderInterfaceDefEntry>,
}

unsafe impl ShaderInterfaceDef for Input {
    type Iter = InputIter;

    fn elements(&self) -> InputIter {
        self.inputs.clone().into_iter()
    }
}

pub type InputIter = std::vec::IntoIter<ShaderInterfaceDefEntry>;

#[derive(Debug, Clone, Default)]
pub struct Output {
    pub outputs: Vec<ShaderInterfaceDefEntry>,
}

unsafe impl ShaderInterfaceDef for Output {
    type Iter = OutputIter;

    fn elements(&self) -> OutputIter {
        self.outputs.clone().into_iter()
    }
}

pub type OutputIter = std::vec::IntoIter<ShaderInterfaceDefEntry>;

#[derive(Debug, Clone, Default)]
pub struct Layout {
    pub layout_data: LayoutData,
}
impl Layout {
    const STAGES: ShaderStages = ShaderStages {
     vertex: false,
     tessellation_control: false,
     tessellation_evaluation: false,
     geometry: false,
     fragment: true,
     compute: false,
    };
}

unsafe impl PipelineLayoutDesc for Layout {
    fn num_sets(&self) -> usize {
        self.layout_data.num_sets
    }
    fn num_bindings_in_set(&self, set: usize) -> Option<usize> {
        self.layout_data.num_bindings.get(&set).map(|&b|b)
    }
    fn descriptor(&self, set: usize, binding: usize) -> Option<DescriptorDesc> {
        self.layout_data.descriptions.get(&set)
            .and_then(|s|s.get(&binding))
            .map(|desc| {
                let mut desc = desc.clone();
                desc.stages = Layout::STAGES;
                desc
            })

    }
    fn num_push_constants_ranges(&self) -> usize {
        self.layout_data.num_constants
    }
    fn push_constants_range(&self, num: usize) -> Option<PipelineLayoutDescPcRange> {
        self.layout_data.pc_ranges.get(num)
            .map(|desc| {
                let mut desc = *desc;
                desc.stages = Layout::STAGES;
                desc
            })

    }
}