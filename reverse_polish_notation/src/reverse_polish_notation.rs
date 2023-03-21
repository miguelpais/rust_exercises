pub mod expression;

use expression::Expression;

pub fn compute(input: String) -> f64 {
    let expr = Expression::parse(input);
    expr.compute_result()
}
