use std::time::Instant;

use crate::{core::storage::GameStorage, utils::window::Window};
use std::{collections::{BinaryHeap, HashMap}, sync::Mutex, collections::VecDeque};
use crate::core::input::{InputGamepad, InputPC};
use winit::{dpi::LogicalSize, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use async_trait::*;


struct AppState{
    pub game_storage: GameStorage,
    // pub renderer: Renderer,
    pub window: Window,

    // timer
    clock: Instant,
    last_frame: Instant,
    pub frame_time: f32,
    pub max_frame_time: f64,
    pub delta_time: f32,
    pub update_per_sec: u32,


    // // events
    // // ring buffer
    // pub events:VecDeque<Box<dyn Event>>,

    // // objects
    // pub objects:HashMap<u64,u64>,

    // system -> physics system, render system
    // system should have a priority, so we used priority queue
    // for example the first one to execute should be physics engine...
    // pub systems: BinaryHeap<Box<dyn System<G,W>>>,
}

#[async_trait]
pub trait Application{
    type Setup;
    // don't block~
    // for example -> (input,output,storage ...)
    async fn setup()->Self::Setup;
    fn cleanup(){}
    fn update(){}
    fn next_frame(){}
    async fn run() {}
}