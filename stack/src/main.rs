pub mod stack;

use stack::Stack;

fn main() {
    let mut stack = Stack::from(3).push(3).push(4).push(5);
    stack.pop();
    stack.pop();
    println!("{:?}", stack);
}
