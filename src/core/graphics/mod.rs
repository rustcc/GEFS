use std::{any::TypeId, borrow::Cow, collections::HashMap, sync::Arc, path::Path};

use async_std::path::PathBuf;
use shaderc::ShaderKind;
use wgpu::ShaderModule;

pub mod resource_manager;
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
#[derive(Debug,Copy, Clone)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
}
impl Into<ShaderKind> for ShaderStage{
    fn into(self) -> ShaderKind {
        match self{
            ShaderStage::Vertex => ShaderKind::Vertex,
            ShaderStage::Fragment => ShaderKind::Fragment,
            ShaderStage::Compute => ShaderKind::Compute,
        }
    }
}

pub struct Shader{
    stage: ShaderStage,
    name: String,
    module: wgpu::ShaderModule,
}


async fn compile_shader<T:Into<PathBuf>>(path:T,kind: ShaderStage,name:&String,device: Arc<wgpu::Device>) -> Option<ShaderModule>{
    let path = path.into();
    let mut compiler = shaderc::Compiler::new()?;
    let mut options = shaderc::CompileOptions::new()?;
    #[cfg(not(debug_assertions))]
    options.set_optimization_level(shaderc::OptimizationLevel::Performance);
    #[cfg(debug_assertions)]
    options.set_optimization_level(shaderc::OptimizationLevel::Zero);
    

    options.set_include_callback(|file_path, _include_type, _, _| {
        let shader_path = path.join(file_path);
        let contents = std::fs::read_to_string(&shader_path).unwrap();
        Result::Ok(shaderc::ResolvedInclude {
            resolved_name: file_path.to_string(),
            content: contents,
        })
    });
    // virtual file storage
    let string = async_std::fs::read_to_string(&path).await.ok()?;
    let spirv = compiler
                .compile_into_spirv(
                    &string,
                    kind.into(),
                    name.as_str(),
                    "main",
                    Some(&options),
                ).ok()?;

                Some(device.create_shader_module(wgpu::ShaderModuleSource::SpirV(Cow::Borrowed(spirv.as_binary()))))
}

impl Shader{
    pub async fn new<T:Into<PathBuf>,S:Into<String>>(device: Arc<wgpu::Device>,name:S,path:T,kind:ShaderStage) -> Option<Arc<Self>>{
        let name = name.into();
        let s = compile_shader(path, kind, &name, device).await?;
        Some(Arc::new(Self{
            stage:kind,
            name,
            module:s
        }))
    }
}

#[test]
fn test_shader_compile(){
    wgpu_subscriber::initialize_default_subscriber(Some(Path::new("./tracing")));
    async_std::task::block_on(async {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let adapter = instance
            .request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::Default,
                    compatible_surface: None,
                },
            )
            .await
            .unwrap();

        let adapter_features = adapter.features();
        let (device, _) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: adapter_features,
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .unwrap();

        let device = Arc::new(device);
        Shader::new(device,"test_triangle.frag", "./assets/shaders/test_tri.frag",ShaderStage::Fragment).await.unwrap();
    });
}

pub struct Shaders{
    shaders: HashMap<String,Shader>
}
impl Shaders{
    pub async fn build_from_config<T:Into<PathBuf>>(path:T){
        
    }
}