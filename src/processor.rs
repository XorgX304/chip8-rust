pub struct Processor {
    register: [u8; 16], // main register
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            register: [0; 16],
        }
    }
}
