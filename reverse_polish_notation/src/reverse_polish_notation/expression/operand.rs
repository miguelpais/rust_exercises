#[derive(Debug)]
pub struct Operand {
    value: f64
}

impl Operand {
    pub fn from(value: f64) -> Operand {
        Operand {
            value
        }
    }

    pub fn value(self: Operand) -> f64 {
        self.value
    }
}
