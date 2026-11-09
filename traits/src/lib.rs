
// trait object that will be inherited across different types
pub trait Draw {
    fn draw(&self);
}


pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// implementing the trait
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

// the compiler will error if a struct that implements a trait object
// does not implement all the required methods of the trait
impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}