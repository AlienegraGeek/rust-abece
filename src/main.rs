fn main() {
    println!("Hello, world!");

    let a = 12;
    println!("a is {}", a);
    println!("{{}}");
    runoob();
    another_function(5, 6);
    for_test();
    dangle2();
    // let reference_to_nothing = dangle();
    // println!("danger is {}", reference_to_nothing);
}

fn runoob() {
    let a = 12;
    println!("a is {}", a);
}

/// fn another_function() {
///     println!("Hello, runoob!");
/// }

fn another_function(x: i32, y: i32) {
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn for_test() {
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("值为 : {}", i);
    }
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn dangle2() {
    let mut s1 = String::from("run");
    // s1 是可变的

    let s2 = &mut s1;
    // s2 是可变的引用

    s2.push_str("oob");
    println!("{}", s2);
}
