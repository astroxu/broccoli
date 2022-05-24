#![allow(unused_variables)] // 使编译器忽律未使用的变量 编译器属性标识 https://course.rs/profiling/compiler/attributes.html
extern crate core;

//use core::panicking::panic;
use num::complex::Complex;
use std::fmt::Debug;
use std::io;
use std::process::Output;

type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
// fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
//     unimplemented!()
// }

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

fn main_20220519() {
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

fn main_20220520() {
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

fn greet(name: String) {
    println!("Hello,{}!", name);
}

fn first_word(s: &String) -> &str {
    &s[..1]
}

fn say_hello(s: &str) {
    println!("{}", s);
}

fn calculate_length1(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct FFile {
    name: String,
    data: Vec<u8>,
}

// struct Users {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
enum PokerSuit {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

// struct PokerCard {
//     suit: PokerSuit,
//     value: u8,
// }

fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main_20220523() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    //read(&mut f1, &mut vec![]);
    close(&mut f1);

    let my_name = "Pascal";
    greet(my_name.to_string());

    let s = String::from("hello world");
    let len = s.len();

    let hello = &s[0..5];
    let hello1 = &s[..5];
    let world = &s[6..11];
    let world1 = &s[6..len];
    let world2 = &s[6..];

    let hw = &s[..];

    let mut s = String::from("test");
    let word = first_word(&s);
    //s.clear();
    println!("the first word is:{}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    let s = String::from("hel");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());

    let hello = "中国人";
    //let s = &hello[0..2];

    let mut ss = String::from("test");
    ss.push('r');
    println!("追加 {}", ss);

    ss.push_str("fdsf");
    println!("追加 {}", ss);

    ss.insert(5, ',');
    println!("insert {}", ss);

    ss.insert_str(0, "ttt ");
    println!("insert {}", ss);

    let str_rep = String::from("heheda");
    let new_str = str_rep.replace("da", "ka");
    dbg!(new_str);

    let new_str2 = str_rep.replacen("he", "te", 1);
    dbg!(new_str2);

    let mut s = String::from("test");
    s.replace_range(0..2, "T");
    dbg!(s);

    let mut s = String::from("test");
    let p1 = s.pop();
    let p2 = s.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(s);

    let mut s = String::from("test");
    println!("str {}", std::mem::size_of_val(s.as_str()));

    s.remove(0);
    dbg!(s);

    let mut s = String::from("test");
    s.truncate(1);
    dbg!(s);

    let mut s = String::from("test");
    s.clear();
    dbg!(s);

    let s1 = String::from("test");
    let s2 = String::from("test");
    let re = s1 + &s2;
    let mut re = re + "!";
    re += "!!!";
    println!("{}", re);

    let s1 = String::from("test");
    let s = format!("{}", s1);
    println!("{}", s);

    let byte = "I'm \x52";
    println!("doing\x3F {}", byte);

    let uni = "\u{211d}";
    let car = "\"\"";
    println!("{} {}", uni, car);

    let long_str = "\
    first_word\
    fsdfdf";
    println!("{}", long_str);

    let quotes = r#""sfs dfs  dfsdf!""#;
    println!("{}", quotes);

    for c in "中国人".chars() {
        println!("{}", c);
    }

    for b in "中国人".bytes() {
        println!("{}", b);
    }

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{}", y);

    let five_h = tup.0;
    let sixpf = tup.1;
    let one = tup.2;

    let s = String::from("test");
    let (s2, len) = calculate_length1(s);
    println!("{} {}", s2, len);

    let mut u1 = User {
        active: true,
        username: "username".to_string(),
        email: "sdsdsd".to_string(),
        sign_in_count: 1,
    };
    u1.email = "fsdsdf".to_string();

    let u2 = User {
        active: u1.active,
        username: u1.username,
        email: "ssdfsdfsd".to_string(),
        sign_in_count: u1.sign_in_count,
    };

    // let u2 = User{
    //     email: "ssdfsdfsd".to_string(),
    //     ..u1
    // };

    let f1 = FFile {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();
    println!("{:?}", f1);
    println!("{} {}", f1_name, f1_length);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // struct AlwaysEqual;
    // let subject = AlwaysEqual;
    // imp SomeTrait for AlwaysEqual{
    //
    // }

    // let  u1 = Users {
    //     active: true,
    //     username: "username",
    //     email: "sdsdsd",
    //     sign_in_count: 1,
    // };

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(3 * scale),
        height: 50,
    };

    dbg!(&rect1);

    // let heart = PokerSuit::Hearts;
    // let diamond = PokerSuit::Diamonds;
    //
    // print_suit(heart);
    // print_suit(diamond);
    //
    // let c1 = PokerCard {
    //     suit: PokerSuit::Clubs,
    //     value: 1,
    // };
    // let c2 = PokerCard {
    //     suit: PokerSuit::Diamonds,
    //     value: 12,
    // };

    let c1 = PokerSuit::Spades(5);
    let c2 = PokerSuit::Diamonds(13);

    //prelude
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [2; 5];

    let first = a[0];
    let second = a[1];

    // println!("Please");
    // let mut index = String::new();
    // io::stdin().read_line(&mut index).expect("Error reading");
    //
    // let index: usize = index.trim().parse().expect("Error parsing");
    // let element = a[index];
    //
    // println!("index{} is: {}", index, element);

    let slice: &[i32] = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    let arrays: [[i32; 5]; 4] = [a, a, a, a];

    for a in &arrays {
        println!("{:?}: ", a);

        for n in a.iter() {
            print!("\t{}+10 ={}", n, n + 10);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }

        println!("\t{:?} = {}", a, sum);
    }
}

enum Direction {
    East,
    West,
    North,
    South,
}

enum IpAddr {
    Ipv4,
    Ipv6,
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

enum MyEnum {
    Foo,
    Bar,
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("{}:{}", x, y);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main_20220524() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("the v = {}", number);

    for i in 1..=5 {
        if i == 2 {
            continue;
        }
        if i == 3 {
            break;
        }
        println!("{}", i)
    }

    let a = [4, 3, 2, 1];
    for (i, v) in a.iter().enumerate() {
        println!("{}:{}", i + 1, v);
    }

    let mut n = 0;
    while n <= 5 {
        println!("{}", n);
        n = n + 1;
    }

    println!("weawea");

    let mut n = 0;
    loop {
        if n > 5 {
            break;
        }
        println!("{}", n);
        n += 1;
    }

    println!("weer");

    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("so");
        }
        _ => println!("west"),
    };

    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("{}", ip_str);

    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("{},{}", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!("{};{};0", r, g,)
            }
        }
    }

    let v = Some(3u8);
    match v {
        Some(3) => println!("three"),
        _ => (),
    };

    if let Some(3) = v {
        println!("three");
    }

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    v.iter().filter(|x| matches!(x, MyEnum::Foo));

    let foo = 'f';
    assert!(matches!(foo,'A'..='Z'|'a'..='z'));

    let bar = Some(4);
    assert!(matches!(bar,Some(x) if x>2));

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let point = (3, 5);
    print_coordinates(&point);

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("quit");
        }
        Message::Move { x, y } => {
            println!("{}:{}", x, y);
        }
        Message::Write(text) => println!("Text {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("{}:{}:{}", r, g, b);
        }
    }

    let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    println!("x:{},y:{}", px, py);
    println!("{:?}", p);

    let point = Point { x: 10, y: 5 };
    if let p @ Point { x: 10, y } = point {
        println!("{}:{:?}", y, p);
    } else {
        println!("efdfss");
    }

    match 1 {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => {}
    }
}

struct Circle {
    x: f64,
    y: f64,
    redius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, redius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            redius: redius,
        }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.redius * self.redius)
    }
}

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub struct Rectangle1 {
    width: u32,
    height: u32,
}

impl Rectangle1 {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle1 { width, height }
    }
    pub fn width(&self) -> u32 {
        return self.width;
    }
}

fn add2<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// fn largest1<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

struct Point1<T> {
    x: T,
    y: T,
}

fn display_array<T: std::fmt::Debug, const N: usize>(arr: &[T; N]) {
    println!("{:?}", arr);
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{} ll", rect1.area());

    let rect1 = Rectangle1::new(30, 50);
    println!("{} ll", rect1.width());

    println!("i8: {}", add2(2i8, 3i8));
    println!("i32{}", add2(20, 30));
    println!("f64{}", add2(1.23, 1.23));

    let integer = Point1 { x: 5, y: 10 };
    let float = Point1 { x: 1.0, y: 4.0 };

    let arr: [i32; 3] = [1, 2, 3];
    display_array(&arr);

    let arr: [i32; 2] = [1, 2];
    display_array(&arr);
}
