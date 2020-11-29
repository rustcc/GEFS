use libgefs::core::input::{InputGamepad, InputPC};

async fn setup() {}
fn run() {
    use gilrs::Gilrs;

    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }
    let mut gamepad_stat = InputGamepad::new();
    let mut pcinput_stat = InputPC::new();

    use winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    event_loop.run(move |event, _, control_flow| {
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
                    // println!("{:?} New event from {}: {:?}", time, id, event);
                    gamepad_stat.update_events(&gilrs::Event { id, event, time });
                    println!("{:?}",gamepad_stat);
                    
                }
                // println!("{:?}",pcinput_stat);
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Redraw the application.
                // println!("redraw~");
            }
            e => {
                pcinput_stat.update_events(&e);
                
            },
        }
    });
}
fn main() {
    println!("Hello, world!");
    run();
}
