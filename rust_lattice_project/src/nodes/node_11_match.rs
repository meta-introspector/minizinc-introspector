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