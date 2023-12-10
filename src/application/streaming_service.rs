pub struct StreamingService { 
    name : String, 
    logged_in: bool,
}

impl StreamingService { 
    pub fn new(name : String) -> Self { 
        Self { 
            name,
            logged_in : false,
        }
    }

    pub fn log_in(&mut self, username : String, password : String) { 
        self.logged_in = true;
    }

    pub fn get_name(&self) -> &str { 
        &self.name
    }

    pub fn is_user_logged_in(&self) -> bool { 
        self.logged_in
    }
}