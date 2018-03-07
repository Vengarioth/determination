use ::dynamics::body::Body;
use ::dynamics::container::Container;

pub struct World {
    container: Container,
}

impl World {
    pub fn new() -> World {
        World {
            container: Container::new(),
        }
    }

    pub fn create_snapshot(&self) {
        
    }

    pub fn apply_snapshot(&mut self) {

    }

    pub fn simulate(&mut self) {

    }

    pub fn add_body(&mut self, body: Body) {
        self.container.add_body(body);
    }

    pub fn ray_cast(&self) {

    }

    pub fn box_cast(&self) {

    }

    pub fn box_query(&self) {

    }
}
