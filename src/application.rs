mod playlist;
mod streaming_service;
use crossterm::event::{self, KeyEvent};
use streaming_service::StreamingService;


#[derive(Debug)]
pub struct Application {
    pub should_quit: bool,
    pub streaming_services: Vec<StreamingService>,
    pub state: [u8; 2],
}

impl Application {
    pub fn new(service_names: Vec<&str>) -> Self {
        let mut services: Vec<StreamingService> = Vec::new();
        service_names.iter().for_each(| &name| 
            services.push(StreamingService::new(name.to_string())));
        Self {
            should_quit: false,
            streaming_services: services,
            state: [0,0]
        }
    }

    pub fn get_state(&self) -> [u8; 2] {
        self.state
    }

    pub fn set_state(&mut self, state: [u8; 2]) {
        self.state = state;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
  
}
