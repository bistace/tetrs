use crate::engine::Engine;

pub struct Interface {
    engine: Engine,
}

impl Interface {
    // Constructs a new Interface with a default engine
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
        }
    }

    // Constructs a new Interface with a user-defined engine
    pub fn with_engine(engine: Engine) -> Self {
        Self { engine }
    }

    pub fn run(self) {
        todo!("Not implemented")
    }
}
