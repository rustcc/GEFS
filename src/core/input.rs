use std::collections::HashSet;
use winit::event::{MouseScrollDelta, VirtualKeyCode};
use winit::event::MouseButton;
use nalgebra_glm::Vec2;

#[derive(Debug)]
pub struct InputPC{
    keys_down: HashSet<VirtualKeyCode>,
    mouse_buttons_down: HashSet<MouseButton>,

    // mouse position
    mouse_position: Vec2,
    // mouse delta
    mouse_delta: Vec2,
    // mouse wheel
    mouse_wheel_delta: Vec2,
}

impl InputPC {
    pub fn new() -> Self {
        Self {
            keys_down: HashSet::new(),
            mouse_buttons_down: HashSet::new(),
            mouse_position: Vec2::zeros(),
            mouse_delta: Vec2::zeros(),
            mouse_wheel_delta: Vec2::zeros(),
        }
    }

    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        self.mouse_buttons_down.contains(&button)
    }

    pub fn update_events(&mut self, winit_event: &winit::event::Event<'_, ()>) {
        match winit_event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == winit::event::ElementState::Pressed {
                        if input.virtual_keycode.is_some() {
                            self.keys_down.insert(input.virtual_keycode.unwrap());
                        }
                    } else if input.state == winit::event::ElementState::Released {
                        if input.virtual_keycode.is_some() {
                            self.keys_down.remove(&input.virtual_keycode.unwrap());
                        }
                    }
                }
                winit::event::WindowEvent::MouseInput {
                    device_id: _,
                    state,
                    button,
                    ..
                } => {
                    match button{
                        MouseButton::Other(_) => {},
                        button => {
                            if *state == winit::event::ElementState::Pressed {
                                self.mouse_buttons_down.insert(*button);
                            } else if *state == winit::event::ElementState::Released {
                                self.mouse_buttons_down.remove(&button);
                            }
                        }
                    }
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    self.mouse_position = Vec2::new(position.x as f32, position.y as f32);
                }
                _ => (),
            },
            winit::event::Event::DeviceEvent { event, .. } => match event {
                winit::event::DeviceEvent::MouseMotion { delta } => {
                    self.mouse_delta = Vec2::new(delta.0 as f32, delta.1 as f32);
                }
                winit::event::DeviceEvent::MouseWheel{delta} => {
                    match delta {
                        MouseScrollDelta::LineDelta(x,y) => {
                            self.mouse_wheel_delta = Vec2::new(*x,*y);
                        },
                        _ => {
                            eprintln!("Sorry, touch pad is unsupported right now.");
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    pub fn clear(&mut self) {
        self.mouse_wheel_delta= Vec2::zeros();
        self.mouse_delta = Vec2::zeros();
    }
}

// currently only support one gamepad
use gilrs::Button;
#[derive(Debug)]
pub struct InputGamepad{
    keys_down: HashSet<Button>,

    left_trigger2_value: f32,
    right_trigger2_value:f32,

    left_axis_value: Vec2,
    right_axis_value: Vec2,
}

impl InputGamepad{
    pub fn new() -> Self {
        Self {
            keys_down: HashSet::new(),
            left_trigger2_value :0.0,
            right_trigger2_value : 0.0,
            left_axis_value : Vec2::zeros(),
            right_axis_value : Vec2::zeros(),
        }
    }
    pub fn clear(&mut self) {
        self.left_trigger2_value = 0.0;
        self.right_trigger2_value = 0.0;
        self.left_axis_value = Vec2::zeros();
        self.right_axis_value = Vec2::zeros();
    }
    pub fn update_events(&mut self, event: &gilrs::Event){
        let a = event.event;
        use gilrs::EventType;
        use gilrs::Axis::*;
        match a{
            EventType::AxisChanged(LeftStickX,x,c) => {
                self.left_axis_value = Vec2::new(x,self.left_axis_value.y)
            },
            EventType::AxisChanged(LeftStickY,y,c) => {
                self.left_axis_value = Vec2::new(self.left_axis_value.x,y)
            },
            EventType::AxisChanged(RightStickX,x,c) => {
                self.right_axis_value = Vec2::new(x,self.right_axis_value.y)
            },
            EventType::AxisChanged(RightStickY,y,c) => {
                self.right_axis_value = Vec2::new(self.right_axis_value.x,y)
            },
            EventType::ButtonPressed(btn,c) => {
                self.keys_down.insert(btn);
            },
            EventType::ButtonReleased(btn,c) => {
                self.keys_down.remove(&btn);
            },
            EventType::ButtonChanged(LeftTrigger2,v,c) => {
                self.left_trigger2_value = v;
            },
            EventType::ButtonChanged(RigherTrigger2,v,c) => {
                self.right_trigger2_value = v;
            },
            _ => {} 
        }
    } 
}