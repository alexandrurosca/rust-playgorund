use std::env;


fn sum(val1: &f32, val2: &f32) -> f32 {
    val1 + val2
}

fn diff(val1: &f32, val2: &f32) -> f32 {
    val1 - val2
}

fn multiply(val1: &f32, val2: &f32) -> f32 {
    val1 * val2
}

fn divide(val1: &f32, val2: &f32) -> f32 {
    if *val2 == 0.0 {
        println!("second value cannot be 0");
        return 0.0
    }
    val1 / val2
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let number1 = &args[1];
    let number2 = &args[2];
    let operator = &args[3];



    let val1 = number1.parse::<f32>().unwrap_or(0.0);
    let val2 = number2.parse::<f32>().unwrap_or(0.0);

    println!("number1: {}", val1);
    println!("number2: {}", val2);
    println!("operator: {}", operator);

    let mut result: f32 = 0.0;
    let sum_operation = String::from("+");
    let diff_operation = String::from("-");
    let multiply_operation = String::from("*");
    let divide_operation = String::from("/");
    let op = String::from(operator);

    if op.eq(&sum_operation) {
        result = sum(&val1, &val2);
    } else if op.eq(&diff_operation) {
        result = diff(&val1, &val2);
    }else if op.eq(&multiply_operation) {
        result = multiply(&val1, &val2);
    }else if op.eq(&divide_operation) {
        result = divide(&val1, &val2);
    } else {
        println!("Unknow operation")
    }

    println!("result: {}", &result);



}