use std::{f32::consts::PI, mem, sync::Arc};

use bytemuck::cast_ref;
use nalgebra::Matrix4;
use nalgebra_glm;
use wgpu::{VertexAttributeDescriptor, util::DeviceExt};
use winit::{dpi::{LogicalSize, PhysicalSize}, event::{Event, WindowEvent}, event_loop::ControlFlow};

use crate::{application::AppState, core::{graphics::{GraphicsBackend, pipeline::{PipelineDescriptor, VertexBufferDescriptor, VertexStateBuilder}, pipeline_manager::{CommandQueueItem, PipelineManager, PipelineType}, resource_manager::GPUResourceManager, shader_manager::ShaderManager}, input::{InputGamepad, InputPC}}, utils::window::Window};

use bytemuck_derive::*;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    _pos: [f32; 4],
    _tex_coord: [f32; 2],
}

fn vertex(pos: [i8; 3], tc: [i8; 2]) -> Vertex {
    Vertex {
        _pos: [pos[0] as f32, pos[1] as f32, pos[2] as f32, 1.0],
        _tex_coord: [tc[0] as f32, tc[1] as f32],
    }
}

fn create_vertices() -> (Vec<Vertex>, Vec<u16>) {
    let vertex_data = [
        // top (0, 0, 1)
        vertex([-1, -1, 1], [0, 0]),
        vertex([1, -1, 1], [1, 0]),
        vertex([1, 1, 1], [1, 1]),
        vertex([-1, 1, 1], [0, 1]),
        // bottom (0, 0, -1)
        vertex([-1, 1, -1], [1, 0]),
        vertex([1, 1, -1], [0, 0]),
        vertex([1, -1, -1], [0, 1]),
        vertex([-1, -1, -1], [1, 1]),
        // right (1, 0, 0)
        vertex([1, -1, -1], [0, 0]),
        vertex([1, 1, -1], [1, 0]),
        vertex([1, 1, 1], [1, 1]),
        vertex([1, -1, 1], [0, 1]),
        // left (-1, 0, 0)
        vertex([-1, -1, 1], [1, 0]),
        vertex([-1, 1, 1], [0, 0]),
        vertex([-1, 1, -1], [0, 1]),
        vertex([-1, -1, -1], [1, 1]),
        // front (0, 1, 0)
        vertex([1, 1, -1], [1, 0]),
        vertex([-1, 1, -1], [0, 0]),
        vertex([-1, 1, 1], [0, 1]),
        vertex([1, 1, 1], [1, 1]),
        // back (0, -1, 0)
        vertex([1, -1, 1], [0, 0]),
        vertex([-1, -1, 1], [1, 0]),
        vertex([-1, -1, -1], [1, 1]),
        vertex([1, -1, -1], [0, 1]),
    ];

    let index_data: &[u16] = &[
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    (vertex_data.to_vec(), index_data.to_vec())
}



fn generate_matrix(aspect_ratio: f32) -> Matrix4<f32> {
    let opengl_to_wgpu_matrix: Matrix4<f32> = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0,
    );
    let mx_projection = nalgebra_glm::perspective(aspect_ratio, 45f32 * PI / 180f32,1.0, 10.0);
    let mx_view = nalgebra_glm::look_at(
        &nalgebra_glm::Vec3::new(1.5f32, -5.0, 3.0),
        &nalgebra_glm::Vec3::new(0f32, 0.0, 0.0),
        &nalgebra_glm::Vec3::new(0.0,0.0,1.0),
    );
    let mx_correction = opengl_to_wgpu_matrix;
    mx_correction * mx_projection * mx_view
}

pub async fn test() {
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
    shader_manager.add_from_config(device.clone(), "./assets/shaders/test_cube").await.unwrap();
    // println!("{:?}",shader_manager.shader_manager);
    let mut pipeline_manager = PipelineManager::new();


    let vertex_size = mem::size_of::<Vertex>();
    let (vertex_data, index_data) = create_vertices();

    let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertex_data),
        usage: wgpu::BufferUsage::VERTEX,
    });

    let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&index_data),
        usage: wgpu::BufferUsage::INDEX,
    });

    //TODO: lib
    // Create pipeline layout
    let mut bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer {
                    min_binding_size: wgpu::BufferSize::new(64),
                    dynamic: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::SampledTexture {
                    multisampled: false,
                    // sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    dimension: wgpu::TextureViewDimension::D2,
                    component_type: wgpu::TextureComponentType::Float,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Sampler {
                    comparison: false,
                },
                count: None,
            },
        ],
    });
    let size = 256u32;
    // let texels = create_texels(size as usize);
    let texture_extent = wgpu::Extent3d {
        width: size,
        height: size,
        depth: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: texture_extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
    });
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    
    let diffuse_bytes = include_bytes!("../../assets/images/cube/image.png");
    let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
    let diffuse_rgba = diffuse_image.as_rgba8().unwrap();
    
    // TODO: lib
    queue.write_texture(
        wgpu::TextureCopyView {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &diffuse_rgba,
        wgpu::TextureDataLayout {
            offset: 0,
            bytes_per_row: 4 * size,
            rows_per_image: 0,
        },
        texture_extent,
    );

    // Create other resources
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });
    let PhysicalSize{width,height} = window.inner_size();
    let mx_total = generate_matrix(width as f32 / height as f32);
    let mx_ref: &[f32; 16] = cast_ref(mx_total.as_ref());
    let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Uniform Buffer"),
        contents: bytemuck::cast_slice(mx_ref),
        usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
    });

    let index_format = wgpu::IndexFormat::Uint16;

    // Create bind group
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(uniform_buf.slice(..)),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
        label: None,
    });
    

    let d = {
        let size = wgpu::Extent3d { // 2.
            width,
            height,
            depth: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT // 3.
                | wgpu::TextureUsage::SAMPLED,
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor { // 4.
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                compare: Some(wgpu::CompareFunction::LessEqual), // 5.
                lod_min_clamp: -100.0,
                lod_max_clamp: 100.0,
                ..Default::default()
            }
        );

        ( texture, view, sampler )
    };
    let mut gpu = GPUResourceManager::init(device.clone());
    let mut pipeline_desc = PipelineDescriptor::new_default("cube".into());
    pipeline_desc.vertex_state = VertexStateBuilder{
        index_format,
        buffer_desc: vec![VertexBufferDescriptor {
            stride: vertex_size as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attrs: vec![
                VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float4,
                    offset: 0,
                    shader_location: 0,
                },
                VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float2,
                    offset: 4 * 4,
                    shader_location: 1,
                },
            ],
        }]
    };

    pipeline_desc.layouts.push("cube".into());

    gpu.add_bind_group_layout("cube", bind_group_layout);
    //gpu.add_bind_group("cube", bind_group,0);
    let gpu = Arc::new(gpu);

    pipeline_manager.add_pipeline("cube", &pipeline_desc, vec![], device.as_ref(), &shader_manager,gpu.clone() ).await;

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
                    let cube_render_pass = {
                        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                                attachment: &frame.view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                    store: true,
                                },
                            }],
                            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                                attachment: &d.1,
                                depth_ops: Some(wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(1.0),
                                    store: true,
                                }),
                                stencil_ops: None,
                            }),
                        });
                        let pl = pipeline_manager.pipelines.get("cube".into()).unwrap();
                        if let PipelineType::GraphicPipeline(pl) = pl {
                            rpass.push_debug_group("Prepare data for draw.");
                            rpass.set_pipeline(&pl.render_pipeline);
                            rpass.set_bind_group(0, &bind_group, &[]);
                            rpass.set_index_buffer(index_buf.slice(..));
                            rpass.set_vertex_buffer(0, vertex_buf.slice(..));
                            rpass.pop_debug_group();
                            rpass.insert_debug_marker("Draw!");
                        }else{
                            panic!("fuck me");
                        }

                        rpass.draw_indexed(0..index_data.len() as u32, 0, 0..1);

                    };
                let qi = CommandQueueItem{name:"cube".into(),buffer:encoder.finish()};
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
