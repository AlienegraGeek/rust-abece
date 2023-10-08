pub mod util {
    pub fn runoob_func() {
        let a = 12;
        println!("a is {}", a);
    }

    pub fn message() -> String {
        String::from("This is the 2nd module.")
    }
}

pub fn get_size() {
    println!("size is in main");
    crate::top_size(); // 必不可少的 crate 关键字
}

