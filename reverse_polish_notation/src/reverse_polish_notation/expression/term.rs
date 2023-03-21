use super::operand::Operand;
use super::operator::Operator;

#[derive(Debug)]
pub enum Term { OPERATOR(Operator), OPERAND(Operand) }

impl Term {
    pub fn operator(symbol: char) -> Term {
        Term::OPERATOR(Operator::from(symbol))
    }
    pub fn operand(value: f64) -> Term {
        Term::OPERAND(Operand::from(value))
    }
}
