fn calc_sum(x: i32, y: i32) -> i32 {
    x + y
}

fn calc_substract(x: i32, y: i32) -> i32 {
    x - y
}

fn calc_divide(x: i32, y: i32) -> i32 {
    x / y
}

fn calc_multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn main() {
    let sum_result = calc_sum(4, 6);
    let sub_result = calc_substract(9, 12);
    let div_result = calc_divide(256, 16);
    let mult_result = calc_multiply(53, 3);
    println!(
        "The sum value is {} and the sub value is {} and the division value is {} and the multiplication value is {}",
        sum_result, sub_result, div_result, mult_result
    );
}
