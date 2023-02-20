////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: create `math!()` macro.
#[macro_export]
macro_rules! math {
    ($left_literal_operand:literal plus $right_literal_operand:literal) => {
        $left_literal_operand + $right_literal_operand
    };
    (square $num:expr) => {
        $num * $num
    }
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    print_result(math!(3 plus 5));
    print_result(math!(square 2));
}
