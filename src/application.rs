mod playlist;
mod streaming_service;
use crossterm::event::{self, KeyEvent};
use streaming_service::StreamingService;

use crate::event::Event;

#[derive(Debug)]
pub struct Application {
    pub should_quit: bool,
    pub streaming_services: Vec<StreamingService>,
}

impl Application {
    pub fn new(service_names: Vec<&str>) -> Self {
        let mut services: Vec<StreamingService> = Vec::new();
        service_names.iter().for_each(| &name| 
            services.push(StreamingService::new(name.to_string())));
        Self {
            should_quit: false,
            streaming_services: services,
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
  
}
