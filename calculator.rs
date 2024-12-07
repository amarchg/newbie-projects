use std::io;

fn main() {
    println!("Calculator");

    println!("Enter first number: ");
    let mut fnumber = String::new();
    io::stdin()
        .read_line(&mut fnumber)
        .expect("Failed to read a line");
    let fnumber: f64 = fnumber.trim().parse().expect("Please type a number");

    println!("Enter operator: ");
    let mut operator = String::new();
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read a line");
    let operator = operator.trim();

    println!("Enter second number: ");
    let mut snumber = String::new();
    io::stdin()
        .read_line(&mut snumber)
        .expect("Failed to read a line");
    let snumber: f64 = snumber.trim().parse().expect("Please type a number");

    if operator == "+" {
        println!("{:.2}", fnumber + snumber);
    } else if operator == "-" {
        println!("{:.2}", fnumber - snumber);
    } else if operator == "*" {
        println!("{:.2}", fnumber * snumber);
    } else if operator == "/" {
        println!("{:.2}", fnumber / snumber);
    } else {
        println!("Error");
    }
}
