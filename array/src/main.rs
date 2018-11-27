use std::mem;
fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys = &xs[2..4];
    println!("{:?} {:?} {} {}", xs, ys, xs.len(), mem::size_of_val(&xs));

    let s = "Hello ".to_string();
    let ss = String::from("World!");

    // let silce = &ss[0..3];
    // let hw = s + &ss;
    // println!("{} {}", silce, hw);

    // let t = (xs, hw);
    // println!("{:?} {}", t.0, t.1);

    // let s2 = s;
    let s2 = s.clone();
    println!("{} {}", s, s2);
}
