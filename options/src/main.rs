fn division(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    let res = division(6.0, 10.0);
    match res {
        Some(x) => println!("{}", x),
        None => println!("Cannot divide by 0"),
    }
}
