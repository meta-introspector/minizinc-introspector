pub fn run() {
    let x = 1;
    println!("x is: {}", x);
    simple_function();
}

pub fn simple_function() {
    // A simple function
}

pub struct MyStruct {
    field1: i32,
    field2: f64,
}

impl MyStruct {
    pub fn new(field1: i32, field2: f64) -> Self {
        MyStruct { field1, field2 }
    }
}

pub trait MyTrait {
    fn do_something(&self);
}

impl MyTrait for MyStruct {
    fn do_something(&self) {
        println!("Doing something!");
    }
}

pub fn use_vector() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
}

pub fn simple_loop() {
    for i in 0..10 {
        println!("{}", i);
    }
}

pub fn simple_match(value: i32) {
    match value {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Other"),
    }
}

pub fn might_fail(succeed: bool) -> Result<i32, String> {
    if succeed {
        Ok(10)
    } else {
        Err("Failed!".to_string())
    }
}

pub struct GenericStruct<T> {
    value: T,
}

impl<T> GenericStruct<T> {
    pub fn new(value: T) -> Self {
        GenericStruct { value }
    }
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn unsafe_block() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}