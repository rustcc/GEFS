use std::any::TypeId;

use libgefs::{utils::window::Window, core::{input::InputGamepad, storage::GameStorage, input::InputPC}};
use winit::{event::Event, dpi::LogicalSize, event::WindowEvent, event_loop::ControlFlow};



async fn test() {
    use gilrs::Gilrs;
    
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }
    let mut gamepad_stat = InputGamepad::new();
    let mut pcinput_stat = InputPC::new();

    
    let Window{events_loop,window} = Window::init("fuck me", LogicalSize::new(1024, 768)).await;

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
                    println!("{:?}",gamepad_stat);
                    
                }
                // println!("{:?}",pcinput_stat);
                // self.update()
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Redraw the application.
                // println!("redraw~");
                // self.next_frame();
            }
            e => {
                pcinput_stat.update_events(&e);
                
            },
        }
    });
}

fn main() {
    println!("Hello, world!");
    async_std::task::block_on(test());
}
