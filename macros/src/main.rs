use std::collections::HashMap;

macro_rules! x_and_y {
    (x => $e:expr) => (println!("X: {}", $e));
    (y => $e:expr) => (println!("Y: {}", $e));
}

macro_rules! build_fn {
    ($func_name:ident) => (
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    )
}

macro_rules! new_map {
    ($($key: expr => $val: expr)*) => {
        {
            let mut map = HashMap::new();

            $(
                map.insert($key, $val);
            )*

            map
        }
    };
}

macro_rules! calc {
    (eval $e:expr) => {{
        {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    }};

    (eval $e:expr, $(eval $es:expr),+) => {
        {
            calc! {eval $e}
            calc! { $(eval $es),+}
        }
    };
}

fn main() {
    x_and_y!(x => 10);
    x_and_y!(y => 20 + 30);

    build_fn!(Hi_there);
    Hi_there();

    let m = new_map!{
        "one" => 1
        "two" => 2
        "three" => 3
    };

    println!("{:?}", m);


    calc!{
        eval 4 * 5,
        eval 4 + 10,
        eval (10 * 3) - 20
    };
}
