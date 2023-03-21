pub mod reverse_polish_notation;

fn main() {
    println!("Result is: {:?}", reverse_polish_notation::compute(String::from("1353 2/ 4 + 989 * 100000 -")));
}
