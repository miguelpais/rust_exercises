use super::operand::Operand;

#[derive(PartialEq, Eq)]
#[derive(Debug)]
pub enum Operator {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE
}

impl Operator {
    pub fn from(symbol: char) -> Operator {
        match symbol {
            '+' => Operator::ADD,
            '-' => Operator::SUBTRACT,
            '*' => Operator::MULTIPLY,
            '/' => Operator::DIVIDE,
            _ => panic!()
        }
    }

    pub fn apply(self, arg1: Operand, arg2: Operand) -> Operand {
        match self {
            Operator::ADD => Operand::from(arg1.value() + arg2.value()),
            Operator::SUBTRACT => Operand::from(arg1.value() - arg2.value()),
            Operator::MULTIPLY => Operand::from(arg1.value() * arg2.value()),
            Operator::DIVIDE => Operand::from(arg1.value() / arg2.value()),
        }
    }
}
