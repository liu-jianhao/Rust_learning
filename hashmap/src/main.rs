use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new();

    hm.insert(String::from("random"), 12);
    hm.insert(String::from("strings"), 42);

    for (k, v) in &hm {
        println!("{}: {}", k, v);
    }

    hm.remove(&String::from("strings"));

    match hm.get(&String::from("strings")) {
        Some(&n) => println!("{}", n),
        _ => println!("no match"),
    }
}
