use std::{any::TypeId, sync::Arc};

use libgefs::{application::AppState, core::{graphics::{GraphicsBackend, pipeline::{Pipeline, PipelineDescriptor}, pipeline_manager::{CommandQueueItem, PipelineManager, PipelineType}, resource_manager::GPUResourceManager, shader_manager::ShaderManager}, input::InputGamepad, input::InputPC, storage::GameStorage}, utils::window::Window};
use nalgebra::{Point3, Vector3};
use wgpu::SwapChainDescriptor;
use winit::{dpi::LogicalSize, event::Event, event::WindowEvent, event_loop::ControlFlow};

async fn test() {
    use gilrs::Gilrs;

    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }
    let mut gamepad_stat = InputGamepad::new();
    let mut pcinput_stat = InputPC::new();

    let Window {
        events_loop,
        window,
    } = Window::init("fuck me", LogicalSize::new(1024, 768)).await;
    
    let GraphicsBackend {
        surface,
        size,
        instance,
        adapter,
        device,
        queue,
        mut swap_chain,
    } = GraphicsBackend::init(&window).await.unwrap();
    let device = Arc::new(device);
    let mut shader_manager = ShaderManager::new();
    shader_manager.add_from_config(device.clone(), "./assets/shaders/test_tri").await.unwrap();
    // println!("{:?}",shader_manager.shader_manager);
    let mut pipeline_manager = PipelineManager::new();
    
    let gpu = Arc::new(GPUResourceManager::init(device.clone()));
    let pipeline_desc = PipelineDescriptor::new_default("triangle".into());
    pipeline_manager.add_pipeline("triangle", &pipeline_desc, vec![], device.as_ref(), &shader_manager,gpu.clone() ).await;

    let mut app = AppState::new();
    events_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            }
            Event::MainEventsCleared => {
                // Application update code.
                if let Some(gilrs::Event { id, event, time }) = gilrs.next_event() {
                    gamepad_stat.update_events(&gilrs::Event { id, event, time });
                    println!("{:?}", gamepad_stat);
                }
                // println!("{:?}",pcinput_stat);
                // self.update()
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Redraw the application.
                // println!("redraw~");
                // self.next_frame();
                let frame = swap_chain
                    .get_current_frame()
                    .expect("Failed to acquire next swap chain texture")
                    .output;

                    let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                    let tri_render_pass = {
                        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                                attachment: &frame.view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                    store: true,
                                },
                            }],
                            depth_stencil_attachment: None,
                        });
                        let pl = pipeline_manager.pipelines.get("triangle".into()).unwrap();
                        if let PipelineType::GraphicPipeline(pl) = pl {
                            rpass.set_pipeline(&pl.render_pipeline);

                        }else{
                            panic!("fuck me");
                        }
                        rpass.draw(0..3, 0..1);

                    };
                let qi = CommandQueueItem{name:"triangle".into(),buffer:encoder.finish()};
                let subq = pipeline_manager.collect_buffers(&mut vec![qi]);
                
                queue.submit(subq);
                app.count();
                println!("app: {:?}",app);
            }
            e => {
                pcinput_stat.update_events(&e);
            }
            Event::NewEvents(_) => {}
            Event::DeviceEvent { device_id, event } => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
        }
    });
}



fn main() {
    println!("Hello, world!");
    async_std::task::block_on(libgefs::test::cube::test());
}
