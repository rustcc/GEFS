use std::{fmt::Debug, time::Instant};

use crate::{core::storage::GameStorage, utils::window::Window};
use std::{collections::{BinaryHeap, HashMap}, sync::Mutex, collections::VecDeque};
use crate::core::input::{InputGamepad, InputPC};
use winit::{dpi::LogicalSize, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use async_trait::*;


pub struct AppState{
    pub game_storage: GameStorage,
    // timer
    pub start: Instant,
    pub last_frame: Instant,
    pub frame_time: u32,
    pub max_frame_time: u32,
    pub frame_count: u64,
}

impl Debug for AppState{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let now= Instant::now();
        f.debug_struct("AppState")
        .field("start", &self.start)
        .field("last_frame", &self.last_frame)
        // .field("fps", & ((now - self.start).as_millis() as f64 / self.frame_count as f64 / 1000.0))
        .field("frame_time", &self.frame_time)
        .field("max_frame_time", &self.max_frame_time)
        .field("frame_count", &self.frame_count)
        .finish()
    }
}

impl AppState{
    pub fn new()->Self{
        Self{
            game_storage:GameStorage::new(),
            start: Instant::now(),
            last_frame: Instant::now(),
            frame_time:0,
            max_frame_time:0,
            frame_count:0,
        }
    }
    pub fn count(&mut self){
        self.frame_count += 1;
        let now= Instant::now();
        self.frame_time = (now - self.last_frame).as_micros() as u32;
        self.last_frame = now;
        if self.frame_time > self.max_frame_time{
            self.max_frame_time = self.frame_time;
        }
    }
}

#[async_trait]
pub trait Application{
    type Setup;
    // don't block~
    // for example -> (input,output,storage ...)
    async fn setup()-> Self::Setup;
    fn cleanup(){}
    fn update(){}
    fn next_frame(){}
    async fn run() {}
}