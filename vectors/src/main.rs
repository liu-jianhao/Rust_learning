fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    println!("{:?} {} {}", v, v.len(), v.capacity());

    println!("{:?}", v.pop());
}
