use std::{sync::Arc, collections::HashMap};

use wgpu::BindGroup;

pub struct GPUResourceManager {
    pub bind_group_layouts: HashMap<String, Arc<wgpu::BindGroupLayout>>,
    pub bind_groups: HashMap<String, HashMap<u32, Arc<BindGroup>>>,
    pub buffers: HashMap<String, Arc<wgpu::Buffer>>,
}

impl GPUResourceManager{
    pub fn init(device: Arc<wgpu::Device>)->Self{
        let bind_group_layouts = HashMap::new();
        let bind_groups = HashMap::new();
        let buffers = HashMap::new();
        Self{
            bind_group_layouts,
            bind_groups,
            buffers
        }
    }
}