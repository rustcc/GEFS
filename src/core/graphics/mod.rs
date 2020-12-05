
use async_std::path::PathBuf;

pub mod resource_manager;
pub mod shader_manager;
pub mod shader;

struct GraphicsBackend {
    surface: wgpu::Surface,
    size: winit::dpi::PhysicalSize<u32>,
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    swap_chain: wgpu::SwapChain,
}
