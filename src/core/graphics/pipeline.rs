use std::sync::Arc;

use wgpu::BindGroupLayout;

use super::{FRAME_FORMAT, resource_manager::GPUResourceManager, shader::ShaderStage, shader_manager::ShaderManager};
#[derive(Clone,Debug)]
pub struct VertexBufferDescriptor{
    // offset
    pub stride: wgpu::BufferAddress,
    pub step_mode: wgpu::InputStepMode,
    // how attributes packed
    pub attrs : Vec<wgpu::VertexAttributeDescriptor>,
}
#[derive(Clone,Debug)]
pub struct VertexStateBuilder{
    pub index_format: wgpu::IndexFormat,
    pub buffer_desc: Vec<VertexBufferDescriptor>,
}
impl VertexStateBuilder {
    pub fn new() -> Self {
        Self {
            index_format: wgpu::IndexFormat::Uint32,
            buffer_desc: Vec::new(),
        }
    }
}
#[derive(Clone,Debug)]
pub struct PipelineDescriptor {
    pub shader: String,
    pub vertex_state: VertexStateBuilder,
    pub primitive_topology: wgpu::PrimitiveTopology,
    pub color_states: Vec<wgpu::ColorStateDescriptor>,
    pub depth_state: Option<wgpu::DepthStencilStateDescriptor>,
    pub sample_count: u32,
    pub sampler_mask: u32,
    pub alpha_to_coverage_enabled: bool,
    pub layouts: Vec<String>,
    pub front_face: wgpu::FrontFace,
    pub cull_mode: wgpu::CullMode,
    pub depth_bias: i32,
    // if you want to hash it, use other kind of floating point rep
    pub depth_bias_slope_scale: f32,
    pub depth_bias_clamp: f32,
    pub push_constant_ranges: Vec<wgpu::PushConstantRange>,
}


impl PipelineDescriptor{
    pub fn new_default(shader:String)->Self{
        Self {
            shader,
            vertex_state: VertexStateBuilder::new(),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: vec![wgpu::ColorStateDescriptor {
                format: FRAME_FORMAT,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_state: None,
            sample_count: 1,
            sampler_mask: !0,
            alpha_to_coverage_enabled: false,
            layouts: Vec::new(),
            front_face: wgpu::FrontFace::Cw,
            cull_mode: wgpu::CullMode::Back,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0.into(),
            depth_bias_clamp: 0.0.into(),
            push_constant_ranges: Vec::new(),
        }
    }
    pub async fn build(&self,device:&wgpu::Device,shader_manager:&ShaderManager,gpu_resource_manager: &GPUResourceManager)->Pipeline{
        let shader_manager = shader_manager.shader_manager.get(&self.shader).unwrap();
        let vertex_shader = &shader_manager.get(&ShaderStage::Vertex).unwrap().module;
        let vertex_stage = wgpu::ProgrammableStageDescriptor {
            module: vertex_shader,
            entry_point: "main",
        };
        let fragment_shader = &shader_manager.get(&ShaderStage::Fragment).unwrap().module;
        let fragment_stage = Some(wgpu::ProgrammableStageDescriptor {
            module: fragment_shader,
            entry_point: "main",
        });

        let bind_group_layouts = self
            .layouts
            .iter()
            .map(|group_name| {
                gpu_resource_manager.bind_group_layouts.get(group_name)
                    .unwrap()
                    .as_ref()
                    
            }).collect::<Vec<_>>();
        let rasterization_state = wgpu::RasterizationStateDescriptor {
            front_face: self.front_face,
            cull_mode: self.cull_mode,
            depth_bias: self.depth_bias,
            depth_bias_slope_scale: self.depth_bias_slope_scale.into(),
            depth_bias_clamp: self.depth_bias_clamp.into(),
            ..Default::default()
        };
        let primitive_topology = self.primitive_topology;
        let color_states = self.color_states.clone();
        let depth_stencil_state = self.depth_state.clone();
        let vertex_state_builder = self.vertex_state.clone();
        let sample_count = self.sample_count;
        let sample_mask = self.sampler_mask;
        let alpha_to_coverage_enabled = self.alpha_to_coverage_enabled;

        // Once we create the layout we don't need the bind group layouts.
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts:&bind_group_layouts.as_slice(), 
            push_constant_ranges: &self.push_constant_ranges.clone(),
            label: None,
        });

        // Creates our vertex descriptor.
        let vertex_buffers: Vec<wgpu::VertexBufferDescriptor<'_>> = vertex_state_builder
            .buffer_desc
            .iter()
            .map(|desc| wgpu::VertexBufferDescriptor {
                stride: desc.stride,
                step_mode: desc.step_mode,
                attributes: &desc.attrs,
            })
            .collect();

        let vertex_state = wgpu::VertexStateDescriptor {
            index_format: vertex_state_builder.index_format,
            vertex_buffers: &vertex_buffers,
        };

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex_stage,
            fragment_stage,
            primitive_topology,
            color_states: &color_states,
            rasterization_state: Some(rasterization_state),
            depth_stencil_state,
            vertex_state,
            sample_count,
            sample_mask,
            alpha_to_coverage_enabled,
        });
        Pipeline{
            desc:self.clone(),
            render_pipeline: pipeline
        }
    }
}
#[derive(Debug)]
pub struct Pipeline {
    pub desc: PipelineDescriptor,
    pub render_pipeline: wgpu::RenderPipeline,
}
