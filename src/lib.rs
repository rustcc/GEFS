pub mod utils;
pub mod core;
pub mod application;

// use futures::task::LocalSpawn;
// #[cfg(not(target_arch = "wasm32"))]
// use std::time::{Duration, Instant};
// use winit::{
//     event::{self, WindowEvent},
//     event_loop::{ControlFlow, EventLoop},
// };

// #[cfg_attr(rustfmt, rustfmt_skip)]
// #[allow(unused)]
// pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
//     1.0, 0.0, 0.0, 0.0,
//     0.0, 1.0, 0.0, 0.0,
//     0.0, 0.0, 0.5, 0.0,
//     0.0, 0.0, 0.5, 1.0,
// );

// pub trait Example: 'static + Sized {
//     fn optional_features() -> wgpu::Features {
//         wgpu::Features::empty()
//     }
//     fn required_features() -> wgpu::Features {
//         wgpu::Features::empty()
//     }
//     fn required_limits() -> wgpu::Limits {
//         wgpu::Limits::default()
//     }
//     fn init(
//         sc_desc: &wgpu::SwapChainDescriptor,
//         device: &wgpu::Device,
//         queue: &wgpu::Queue,
//     ) -> Self;
//     fn resize(
//         &mut self,
//         sc_desc: &wgpu::SwapChainDescriptor,
//         device: &wgpu::Device,
//         queue: &wgpu::Queue,
//     );
//     fn update(&mut self, event: WindowEvent);
//     fn render(
//         &mut self,
//         frame: &wgpu::SwapChainTexture,
//         device: &wgpu::Device,
//         queue: &wgpu::Queue,
//         spawner: &impl LocalSpawn,
//     );
// }

// struct Setup {
//     window: winit::window::Window,
//     event_loop: EventLoop<()>,
//     instance: wgpu::Instance,
//     size: winit::dpi::PhysicalSize<u32>,
//     surface: wgpu::Surface,
//     adapter: wgpu::Adapter,
//     device: wgpu::Device,
//     queue: wgpu::Queue,
// }

// async fn setup<E: Example>(title: &str) -> Setup {
//     // #[cfg(not(target_arch = "wasm32"))]
//     {
//         let chrome_tracing_dir = std::env::var("WGPU_CHROME_TRACE");
//         subscriber::initialize_default_subscriber(
//             chrome_tracing_dir.as_ref().map(std::path::Path::new).ok(),
//         );
//     };

//     let event_loop = EventLoop::new();
//     let mut builder = winit::window::WindowBuilder::new();
//     builder = builder.with_title(title);
//     let window = builder.build(&event_loop).unwrap();

//     log::info!("Initializing the surface...");

//     let backend = if let Ok(backend) = std::env::var("WGPU_BACKEND") {
//         match backend.to_lowercase().as_str() {
//             "vulkan" => wgpu::BackendBit::VULKAN,
//             "metal" => wgpu::BackendBit::METAL,
//             "dx12" => wgpu::BackendBit::DX12,
//             "dx11" => wgpu::BackendBit::DX11,
//             "gl" => wgpu::BackendBit::GL,
//             "webgpu" => wgpu::BackendBit::BROWSER_WEBGPU,
//             other => panic!("Unknown backend: {}", other),
//         }
//     } else {
//         wgpu::BackendBit::PRIMARY
//     };
//     let power_preference = if let Ok(power_preference) = std::env::var("WGPU_POWER_PREF") {
//         match power_preference.to_lowercase().as_str() {
//             "low" => wgpu::PowerPreference::LowPower,
//             "high" => wgpu::PowerPreference::HighPerformance,
//             other => panic!("Unknown power preference: {}", other),
//         }
//     } else {
//         wgpu::PowerPreference::default()
//     };
//     let instance = wgpu::Instance::new(backend);
//     let (size, surface) = unsafe {
//         let size = window.inner_size();
//         let surface = instance.create_surface(&window);
//         (size, surface)
//     };
//     let adapter = instance
//         .request_adapter(&wgpu::RequestAdapterOptions {
//             power_preference,
//             compatible_surface: Some(&surface),
//         })
//         .await
//         .unwrap();

//     // #[cfg(not(target_arch = "wasm32"))]
//     {
//         let adapter_info = adapter.get_info();
//         println!("Using {} ({:?})", adapter_info.name, adapter_info.backend);
//     }

//     let optional_features = E::optional_features();
//     let required_features = E::required_features();
//     let adapter_features = adapter.features();
//     assert!(
//         adapter_features.contains(required_features),
//         "Adapter does not support required features for this example: {:?}",
//         required_features - adapter_features
//     );

//     let needed_limits = E::required_limits();

//     let trace_dir = std::env::var("WGPU_TRACE");
//     let (device, queue) = adapter
//         .request_device(
//             &wgpu::DeviceDescriptor {
//                 label: None,
//                 features: (optional_features & adapter_features) | required_features,
//                 limits: needed_limits,
//                 shader_validation: true,
//             },
//             trace_dir.ok().as_ref().map(std::path::Path::new),
//         )
//         .await
//         .unwrap();

//     Setup {
//         window,
//         event_loop,
//         instance,
//         size,
//         surface,
//         adapter,
//         device,
//         queue,
//     }
// }

// fn start<E: Example>(
//     Setup {
//         window,
//         event_loop,
//         instance,
//         size,
//         surface,
//         adapter,
//         device,
//         queue,
//     }: Setup,
// ) {
//     // #[cfg(not(target_arch = "wasm32"))]
//     let (mut pool, spawner) = {
//         let local_pool = futures::executor::LocalPool::new();
//         let spawner = local_pool.spawner();
//         (local_pool, spawner)
//     };

//     let mut sc_desc = wgpu::SwapChainDescriptor {
//         usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
//         // TODO: Allow srgb unconditionally
//         format: if cfg!(target_arch = "wasm32") {
//             wgpu::TextureFormat::Bgra8Unorm
//         } else {
//             wgpu::TextureFormat::Bgra8UnormSrgb
//         },
//         width: size.width,
//         height: size.height,
//         present_mode: wgpu::PresentMode::Mailbox,
//     };
//     let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

//     log::info!("Initializing the example...");
//     let mut example = E::init(&sc_desc, &device, &queue);

//     // #[cfg(not(target_arch = "wasm32"))]
//     let mut last_update_inst = Instant::now();

//     log::info!("Entering render loop...");
//     event_loop.run(move |event, _, control_flow| {
//         let _ = (&instance, &adapter); // force ownership by the closure
//         *control_flow = if cfg!(feature = "metal-auto-capture") {
//             ControlFlow::Exit
//         } else {
//             // #[cfg(not(target_arch = "wasm32"))]
//             {
//                 ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(10))
//             }
//         };
//         match event {
//             event::Event::MainEventsCleared => {
//                 // #[cfg(not(target_arch = "wasm32"))]
//                 {
//                     if last_update_inst.elapsed() > Duration::from_millis(20) {
//                         window.request_redraw();
//                         last_update_inst = Instant::now();
//                     }

//                     pool.run_until_stalled();
//                 }

//             }
//             event::Event::WindowEvent {
//                 event: WindowEvent::Resized(size),
//                 ..
//             } => {
//                 log::info!("Resizing to {:?}", size);
//                 sc_desc.width = if size.width == 0 { 1 } else { size.width };
//                 sc_desc.height = if size.height == 0 { 1 } else { size.height };
//                 example.resize(&sc_desc, &device, &queue);
//                 swap_chain = device.create_swap_chain(&surface, &sc_desc);
//             }
//             event::Event::WindowEvent { event, .. } => match event {
//                 WindowEvent::KeyboardInput {
//                     input:
//                         event::KeyboardInput {
//                             virtual_keycode: Some(event::VirtualKeyCode::Escape),
//                             state: event::ElementState::Pressed,
//                             ..
//                         },
//                     ..
//                 }
//                 | WindowEvent::CloseRequested => {
//                     *control_flow = ControlFlow::Exit;
//                 }
//                 _ => {
//                     example.update(event);
//                 }
//             },
//             event::Event::RedrawRequested(_) => {
//                 let frame = match swap_chain.get_current_frame() {
//                     Ok(frame) => frame,
//                     Err(_) => {
//                         swap_chain = device.create_swap_chain(&surface, &sc_desc);
//                         swap_chain
//                             .get_current_frame()
//                             .expect("Failed to acquire next swap chain texture!")
//                     }
//                 };

//                 example.render(&frame.output, &device, &queue, &spawner);
//             }
//             _ => {}
//         }
//     });
// }

// #[cfg(not(target_arch = "wasm32"))]
// pub fn run<E: Example>(title: &str) {
//     let setup = futures::executor::block_on(setup::<E>(title));
//     start::<E>(setup);
// }