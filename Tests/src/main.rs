#[cfg(test)]
mod tests {
    #[test]
    fn it_wroks() {

    }

    #[test]
    fn check_two() {
        assert!(1 + 1 == 2);
    }

    #[test]
    #[should_panic]
    fn check_three() {
        assert!(1 + 2 == 4);
    }
}


#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("running linux!");
}
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("not running linux!");
}

fn main() {
    are_you_on_linux();

    println!("check OS again");
    if cfg!(target_os = "linux") {
        println!("definitely linux");
    } else {
        println!("not linux");
    }
}