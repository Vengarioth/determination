use ::dynamics::body::Body;

pub struct Container {
    bodies: Vec<Body>,
}

impl Container {
    pub fn new() -> Container {
        Container {
            bodies: Vec::new(),
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }
}
