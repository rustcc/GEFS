use std::{collections::HashMap, sync::Arc};
use solvent::DepGraph;

use super::{pipeline::*, resource_manager::GPUResourceManager, shader_manager::ShaderManager};

pub struct CommandQueueItem {
    pub name: String,
    pub buffer: wgpu::CommandBuffer,
}

pub type CommandBufferQueue = Vec<CommandQueueItem>;

#[derive(Debug)]
pub enum PipelineType{
    GraphicPipeline(Pipeline),
    // ComputePipeline(),

    // NodeWithOutPipeline,
    // // 
    // ParallelPipeline(Vec<PipelineType>),
}
#[derive(Debug)]
pub struct PipelineManager{
    pub pipelines: HashMap<String,PipelineType>,
    dependency_graph: DepGraph<String>,
    order: Vec<String>,
}

impl PipelineManager{
    pub fn new() -> Self{
        let mut root_dep = DepGraph::new();
        root_dep.register_node("root".to_string());
        Self{
            pipelines: HashMap::new(),
            dependency_graph: root_dep,
            order:vec![]
        }
    }

    pub async fn add_pipeline<T: Into<String>>(
        &mut self,
        name: T,
        pipeline_desc: &PipelineDescriptor,
        dependencies: Vec<&str>,
        device: &wgpu::Device,
        shader_manager: &ShaderManager,
        gpu_resource_manager: Arc<GPUResourceManager>,
    ) {
        let name = name.into();
        let pipeline = pipeline_desc.build(&device,&shader_manager, &gpu_resource_manager).await;
        self.pipelines.insert(name.clone(), PipelineType::GraphicPipeline(pipeline));
        // Add to our graph
        self.dependency_graph.register_node(name.clone());

        if dependencies.len() > 0 {
            let dependencies = dependencies
                .iter()
                .map(|name| name.to_string())
                .collect::<Vec<String>>();
            self.dependency_graph
                .register_dependencies(name, dependencies);
        }

        // Recalculate order.
        self.reorder();
    }

    fn reorder(&mut self) {
        let mut order = Vec::new();
        for (name, _) in self.pipelines.iter() {
            let dependencies = self.dependency_graph.dependencies_of(&name);
            if dependencies.is_ok() {
                for node in dependencies.unwrap() {
                    match node {
                        Ok(n) => {
                            if !order.contains(n) {
                                order.push(n.clone());
                            }
                        }
                        Err(e) => panic!("Solvent error detected: {:?}", e),
                    }
                }
            }
        }

        // UI always comes last.
        order.push("UI".to_string());
        self.order = order;
    }

    pub fn collect_buffers(
        &self,
        command_queue: &mut CommandBufferQueue,
    ) -> Vec<wgpu::CommandBuffer> {
        let mut command_buffers = Vec::new();
        let mut queue_items = Vec::new();
        while let Some(command) = command_queue.pop() {
            queue_items.push(command);
        }

        for order in self.order.iter() {
            while let Some(queue_item_index) = queue_items
                .iter()
                .position(|queue_item| &queue_item.name == order)
            {
                let queue_item = queue_items.remove(queue_item_index);
                command_buffers.push(queue_item.buffer);
            }
        }
        command_buffers
    }
}