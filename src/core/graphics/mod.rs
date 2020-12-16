
use async_std::path::PathBuf;

pub mod resource_manager;
pub mod shader_manager;
pub mod shader;
pub mod pipeline_manager;
pub mod pipeline;
pub mod texture;

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
pub const FRAME_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

pub struct GraphicsBackend {
    pub surface: wgpu::Surface,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub swap_chain: wgpu::SwapChain,
}
impl GraphicsBackend {
    pub async fn init(window:&winit::window::Window)->Option<GraphicsBackend>{
        let size = window.inner_size();
        // primary is dependent on the platform
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe{instance.create_surface(window)};
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions{
                compatible_surface: Some(&surface),
                ..Default::default()
            }
        ).await?;
        let (device,queue) = adapter.request_device(            &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            shader_validation: true,
        },
        None).await.ok()?;
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: FRAME_FORMAT,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        Some(Self{
            surface,
            size,
            instance,
            adapter,
            device,
            queue,
            swap_chain
        })
    }
}