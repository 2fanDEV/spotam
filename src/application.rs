mod playlist;
mod streaming_service;
use std::env;

use ratatui::layout::{Direction, Layout, Rect};
use streaming_service::StreamingService;


#[derive(Debug)]
pub struct Navigation { 
    pub x: u32,
    pub y: u32,
    pub limit_x: u32,
    pub limit_y: u32,
}

impl Navigation { 
    pub fn move_up(&mut self) { 
        if self.y > 0 { 
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self) { 
        if self.y < self.limit_y { 
            self.y += 1;
        }
    }

    pub fn move_left(&mut self) { 
        if self.x > 0 { 
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) { 
        if self.x < self.limit_x { 
            self.x += 1;
        }
    }
}

#[derive(Debug)]
pub struct Application {
    pub should_quit: bool,
    pub streaming_services: Vec<StreamingService>,
    pub navigation: Navigation,
}   

impl Application {
    pub fn new(service_names: Vec<&str>) -> Self {
        let mut services: Vec<StreamingService> = Vec::new();
        service_names.iter().for_each(| &name| 
            services.push(StreamingService::new(name.to_string())));
        
        Self {
            should_quit: false,
            streaming_services: services,
            navigation: Navigation { x: 0, y: 0, limit_x:0, limit_y:0},
        }
    }

    pub fn authorization_flow_spotify(&mut self) { 
        let x = std::env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID not set");
        println!("The value of SPOTIFY_CLIENT_ID is: {}", x);
    }

    pub fn check_if_user_is_logged_in_streaming_services(&self) -> bool { 
        let mut logged_in = false;
        for service in &self.streaming_services { 
            if service.is_user_logged_in() { 
                logged_in = true;
            }
        }
        logged_in
    }


    pub fn quit(&mut self) {
        self.should_quit = true;
    }
  
}
