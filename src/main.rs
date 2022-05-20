extern crate core;

//use core::panicking::panic;
use num::complex::Complex;
use std::fmt::Debug;

fn complex_num() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{}+{}i", result.re, result.im);
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1;
    let y = y + 1;
    x + y
}

fn ret_unit_type() {
    let x = 1;
    if (x > 1) {}
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn another_function(x: i32, y: f32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_five(x: i32) -> i32 {
    x + 5
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }

    x + 5
}

fn report<T: Debug>(item: T) {
    println!("{:?}", item);
}

fn clear(text: &mut String) -> () {
    *text = String::from("");
}

fn add1(x: u32, y: u32) -> u32 {
    x + y
}

fn dead_end() -> ! {
    panic!("崩溃吧！");
}

fn forever() -> ! {
    loop {
        //
    }
}

fn main_20220520() {
    //complex_num();
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&x));

    //add_with_extra(1,2);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is:{}", y);

    assert_eq!(ret_unit_type(), ());

    another_function(5, 6.1);

    let x = plus_five(5);
    println!("The value of x is: {}", x);

    let x = plus_or_minus(5);
    println!("The value of x is: {}", x);

    report("test");
    dead_end();
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}

fn gives_ownership() -> String {
    let some_string = String::from("hel");
    some_string
}

fn takes_and_give_bakc(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(",wor");
}

fn dangle() -> String {
    let s = String::from("hel");

    s
}

fn main() {
    let mut s = String::from("test");
    s.push_str(",tt");
    println!("{}", s);

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{},wew", s2);

    let x1: &str = "helo";
    let y = x;
    println!("{},{}", x, y);

    let s1 = String::from("hel");
    let s2 = s1.clone();
    println!("{},{}", s1, s2);

    let s = String::from("sdfsd");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    let s1 = gives_ownership();
    let s2 = String::from("sdf");
    let s3 = takes_and_give_bakc(s2);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let s1 = String::from("hel");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s1 = String::from("test");
    change(&mut s1);

    {
        let r1 = &mut s1;
    }
    let r2 = &mut s1;

    println!("{}", r2);

    let r1 = &s1;
    let r2 = &s1;
    //let r3 = &mut s1;
    //println!("{},{},{}", r1, r2, r3);

    let reference_to_nothing = dangle();
}
