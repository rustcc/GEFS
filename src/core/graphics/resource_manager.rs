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
    pub fn add_bind_group_layout<T: Into<String>>(
        &mut self,
        name: T,
        bind_group_layout: wgpu::BindGroupLayout,
    ) {
        let name = name.into();
        if self.bind_group_layouts.contains_key(&name) {
            panic!("fuck me!");
        }
        self.bind_group_layouts
            .insert(name, Arc::new(bind_group_layout));
    }
    pub fn add_bind_group<T: Into<String>>(&mut self, key: T, bind_group: BindGroup, id:u32) {
        let key = key.into();
        // let bind_group_index = bind_group.index;
        if self.bind_groups.contains_key(&key) {
            let bind_groups = self.bind_groups.get_mut(&key).unwrap();
            bind_groups.insert(id, Arc::new(bind_group));
        } else {
            let mut hash_map = HashMap::new();
            hash_map.insert(id, Arc::new(bind_group));
            self.bind_groups.insert(key.clone(), hash_map);
        }
    }
}