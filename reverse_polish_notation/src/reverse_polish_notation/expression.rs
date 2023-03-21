pub mod operand;
pub mod operator;
pub mod term;

use crate::reverse_polish_notation::expression::term::Term;

#[derive(Debug)]
pub struct Expression {
    terms: Vec<Term>
}

impl Expression {
    fn terms(self: Expression) -> Vec<Term> {
        self.terms
    }
    fn empty() -> Expression {
        Expression { terms: Vec::new() }
    }

    pub fn parse(input: String) -> Expression {
        let mut expr = Expression::empty();
        let mut carry_over: Option<f64> = None;
        for current_char in input.chars() {
            let current_digit = current_char.to_digit(10);
            match (current_char, carry_over) {
                ('0'..='9', None) => {
                    carry_over = Some(current_digit.unwrap() as f64)
                },
                ('0'..='9', Some(carry)) => {
                    carry_over = Some(carry * 10.0 + current_digit.unwrap() as f64);
                }
                (' ', Some(carry)) => {
                    carry_over = None;
                    expr.terms.push(Term::operand(carry));
                }
                (' ', None) => (),
                ('+' | '-' | '*' | '/', Some(carry)) => {
                    carry_over = None;
                    expr.terms.push(Term::operand(carry));
                    expr.terms.push(Term::operator(current_char));
                }
                ('+' | '-' | '*' | '/', None) => {
                    expr.terms.push(Term::operator(current_char))
                },
                _ => panic!()
            }
        }
        if carry_over.is_some() {
            expr.terms.push(Term::operand(carry_over.unwrap()));
        }
        expr
    }

    pub fn compute_result(self: Expression) -> f64 {
        let mut stack: Vec<Term> = Vec::new();
        for term in self.terms() {
            match term {
                Term::OPERATOR(operator) => {
                    match (stack.pop(), stack.pop()) {
                        (Some(Term::OPERAND(op2)), Some(Term::OPERAND(op1))) => {
                            stack.push(Term::OPERAND(operator.apply(op1, op2)));
                        },
                        _ => panic!()
                    }
                },
                Term::OPERAND(operand) => stack.push(Term::OPERAND(operand))
            }
        }
        match stack.pop() {
            Some(Term::OPERAND(op)) => op.value(),
            _ => panic!()
        }
    }
}
