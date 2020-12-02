use winit::{dpi::LogicalSize, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

pub struct Window{
    pub events_loop: EventLoop<()>,
    pub window: winit::window::Window,
}
impl Window{
    pub async fn init<T:Into<String> >(
        title : T,
        size: LogicalSize<u32>,
    ) -> Self{
        let events_loop = EventLoop::new();
        let window = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(size)
        .build(&events_loop)
        .unwrap();
        Self{
            events_loop,
            window
        }
    }
}