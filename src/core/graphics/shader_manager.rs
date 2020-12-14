use std::{sync::Arc, collections::HashMap};

use std::path::{Path, PathBuf};

use super::shader::{Shader, ShaderStage};


pub struct ShaderManager{
    pub shader_manager: HashMap<String,HashMap<ShaderStage,Arc<Shader>>>,
}
impl ShaderManager{
    pub fn new()->Self{
        Self{
            shader_manager:HashMap::new()
        }
    }
    pub async fn add_from_config<T:Into<PathBuf>>(&mut self,    device: Arc<wgpu::Device>,path:T)->Option<()>{
        // config path
        /*
            {
                name: "string"
                vertex : "path",
                fragment : "path",
                compute: "path"
            }
        */
        let path = path.into();
        let config = std::fs::read_to_string(&path.join("config.json")).ok()?;
        let config:HashMap<String,String> = serde_json::from_str(config.as_str()).ok()?;
        let name = config.get("name".into())?;
        let mut table = HashMap::new();
        if let Some(vert_path) = config.get("vertex".into()){
            let shader = Shader::new(device.clone(), &(format!("{}{}",name,".vert")), path.join(vert_path), ShaderStage::Vertex).await?;
            table.insert(ShaderStage::Vertex,shader);  
        }
        if let Some(frag_path) = config.get("fragment".into()){
            let shader = Shader::new(device.clone(), &(format!("{}{}",name,".frag")), path.join(frag_path), ShaderStage::Fragment).await?;
            table.insert(ShaderStage::Fragment,shader);  
        }
        if let Some(comp_path) = config.get("compute".into()){
            let shader = Shader::new(device.clone(), &(format!("{}{}",name,".comp")), path.join(comp_path), ShaderStage::Compute).await?;
            table.insert(ShaderStage::Compute,shader);  
        }
        self.shader_manager.insert(name.clone(), table);
        
        println!("shader loaded {:?}",self.shader_manager);
        Some(())
    }
}

#[test]
fn test_ShaderManager(){
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
        let mut s = ShaderManager{shader_manager: HashMap::new()};
        s.add_from_config(device, "./assets/shaders/test_tri").await.unwrap();
        assert!(s.shader_manager.get("triangle").is_some() == true);
    });
}