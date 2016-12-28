

pub struct Knob<T> {
    name: &'static str,
    value: Cell<T>,
}

impl Knob<T> {
    pub fn new(name: &'static str, initial_value: T) -> Self {
        Knob {name: name, value: Cell::new(initial_value)}
    }
}

pub trait Knobs {
    pub fn all_knobs() -> impl Copy;
}