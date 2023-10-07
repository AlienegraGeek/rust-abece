fn main() {
    println!("Hello, world!");

    let a = 12;
    println!("a is {}", a);
    println!("{{}}");
    runoob();
    another_function(5, 6);
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