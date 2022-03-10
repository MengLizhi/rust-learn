use num::{complex::Complex, ToPrimitive};
use std::io;

const MAX_POINTS: u32 = 100_000;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn rust_course_1() {
    let one = '一';
    let two = '二';
    let arr = [one, two];
    for item in arr.iter() {
        println!("{}", &item);
    }
}

fn rust_course_2() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // 声明一个fields变量，类型是Vec
        // Vec是vector的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
        // <_>表示Vec中的元素类型由编译器自行推断，在很多场景下，都会帮我们省却不少功夫
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            // 输出到标准错误输出
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        // 1. 尝试把fields[1]的值转换为f32类型的浮点数，如果成功，则把f32值赋给length变量
        // 2. if let是一个匹配表达式，用来从=右边的结果中，匹配出length的值:
        // 当=右边的表达式执行成功，则会返回一个Ok(f32)的类型，若失败，则会返回一个Err(e)类型，if let的作用就是仅匹配Ok也就是成功的情
        // 况,如果是错误，就直接忽略，同时if let还会做一次解构匹配，通过Ok(length)去匹配右边的Ok(f32)，最终把相应的f32值赋给length
        // 3. 当然你也可以忽视成功的情况，用if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
        if let Ok(length) = fields[1].parse::<f32>() {
            // 输出到标准输出
            println!("{}, {}cm", name, length);
        }
    }
}

fn rust_course_3() -> i32 {
    let a = 10;
    let b: i32 = 12;
    let c = 90i32;
    let d = 90_i32;

    let val = add(add(a, b), add(c, d));

    println!("(a + b ) + (c + d) = {}", val);
    return val;

    fn add(i: i32, j: i32) -> i32 {
        i + j
    }
}

fn rust_course_4() {
    let mut x = 5;
    println!("val {}", x);
    x = 89;
    println!("val {}", x);

    let (a, mut b): (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b, "we are testing addition with {} and {}", a, b);

    println!("const MAX_POINTS val {:?}", MAX_POINTS);

    // -------------------------------------
    // let shadowing
    // -------------------------------------
    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}

fn rust_course_5() {
    let x: i128 = "42".parse().expect("Not as Number");

    println!("x val {}", x)
}

fn rust_float_num() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let zxc: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc  0.1 + 0.2 = {:x}", (abc.0 + abc.1).to_bits());
    println!("abc        0.3 = {:x}", (abc.2).to_bits());

    println!("zxc  0.1 + 0.2 = {:x}", (zxc.0 + zxc.1).to_bits());
    println!("zxc        0.3 = {:x}", (zxc.2).to_bits());

    // assert!(abc.0 + abc.1 == abc.2);
    // assert!(zxc.0 + zxc.1 == zxc.2);

    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}

fn rust_nan_num() {
    let x = (-42.0_f32).sqrt();
    println!("num {} is NaN", x.is_nan())

    // assert_eq!(x,x)
}

fn rust_range() {
    let mut t = String::from("");
    for i in 1..=5 {
        // println!("{}", i)
        t.push_str("asd");
    }

    // println!("{}", t);

    let t = String::from("");

    for i in 'a'..='z' {
        // println!("{}", i)
        // t.push_str(i)
    }
    println!("{}", t)
}

fn rust_char() {
    let c = 'z';
    println!(
        "{} 字符占用了{}字节的内存大小",
        c,
        std::mem::size_of_val(&c)
    );
    let z = 'ℤ';
    println!(
        "{} 字符占用了{}字节的内存大小",
        z,
        std::mem::size_of_val(&z)
    );
    let g = '国';
    println!(
        "{} 字符占用了{}字节的内存大小",
        g,
        std::mem::size_of_val(&g)
    );
    let heart_eyed_cat = '😻';
    println!(
        "{} 字符占用了{}字节的内存大小",
        heart_eyed_cat,
        std::mem::size_of_val(&heart_eyed_cat)
    );
}

fn rust_course_6() {
    let mut s = String::from("");

    s.push_str("hello");

    println!("{}", s)
}

fn rust_clone() {
    let a = String::from("aaaaa");
    let _a = a.clone();

    assert_eq!(a, _a);
    println!("a 与 _a 的值是否相同 {}", a == _a);
}

fn rust_shallow_copy() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y)
}

/// Rust 中的“复制”与“移动”解释
/// https://course.rs/basic/ownership/ownership.html#%E5%87%BD%E6%95%B0%E4%BC%A0%E5%80%BC%E4%B8%8E%E8%BF%94%E5%9B%9E
fn rust_copy_move() {
    // s 值初始化
    let s = String::from("hello");
    // s 值移动到 rust_copy_move_s 函数中，函数再将其返回并重新赋值给 s
    let s = rust_copy_move_s(s);

    println!("s val {}", s);

    let n = 5;

    rust_copy_move_n(n);

    println!("n val {}", n);
}

fn rust_copy_move_s(str: String) -> String {
    println!("move val {}", str);

    return str;
}
fn rust_copy_move_n(num: i32) {
    println!("num val {}", num)
}

fn rust_course_7() {
    let a = 9;
    // & 表示引用所有权 （引用）
    let b = &a;

    // assert_eq!(9,b); // i32 类型是无法与 引用（&）进行比较的

    // * 表示解析引用的所有权，来找到真正的值 （解引用）
    assert_eq!(9, *b);
    assert_eq!(a, *b);

    // --------------------
    // 不可变引用
    // --------------------
    let s1 = String::from("hello");
    let length = get_s1_length(&s1);
    fn get_s1_length(t: &String) -> usize {
        t.len()
    }

    println!("{} length val {}", s1, length);

    // --------------------
    // 可变引用
    // --------------------

    let mut s1 = String::from("name");

    let _s1 = &mut s1; // 可变引用
                       // let __s1 = &mut s1; // 同一作用域下只能存在一个可变引用

    // 这个是无法通过编译器检查的，因为先使用了_s1的可变引用，又接着要使用 __s1 可变引用，
    // 存在同一作用域下有多个指向同一个变量的可变引用，术语：borrow checker
    // println!("{} : {}", *_s1, *__s1);

    str_change(&mut s1);
    fn str_change(t: &mut String) {
        t.push_str(" lizhi");
    }

    println!("{}", s1);
}

fn rust_dangling_references_check() {
    // fn dangle () -> &String {
    //     let s = String::from("hello");

    //     &s
    // }

    // let reference_to_nothing = dangle();
}

fn rust_tag_fn() {
    // unimplemented!();
    todo!()
}

fn rust_str_slice() {
    let s = String::from("hello");

    let s1 = &s[0..3];
    let s2 = &s[3..4];

    println!("{}: {}", s1, s2)
}

fn rust_course_8() {
    fn first_word(v: &String) -> &str {
        &v[0..2]
    }
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // 而之后的 println! 又使用了不可变借用，也就是在 s.clear() 处可变借用与不可变借用试图同时生效，因此编译无法通过。
    // s.clear(); // error!

    println!("the first word is: {}", word);
}

fn rust_string() {
    let mut string = String::new();
    let def_string = "hello";
    string.push_str(def_string);
    string.push('!');

    assert_eq!(string, "hello!");

    let mut utf8_string = "嗨".to_string();
    utf8_string.push('！');

    assert_eq!(utf8_string, "嗨！");

    let join_string = string + " " + &utf8_string;

    println!("{}", join_string);
}

fn rust_struct() {
    fn build_user(email: String, username: String) -> User {
        return User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        };
    }

    let mut user = build_user("ccc@aa.com".to_string(), "HHH".to_string());
    println!("{} : {}", user.username, user.email);

    user.username.push_str("CCC");

    println!("{} : {}", user.username, user.email)
}

fn rust_new_struct() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // .. 操作符自动填充剩余结构（struct）属性
    let user3 = User {
        email: "cc@aa.com".to_string(),
        ..user2
    };

    println!("{}", user1.active);
    // 下面这行会报错
    // println!("{:?}", user1);
}

fn rust_struct_monney() {
    #[derive(Debug)]
    struct File {
        name: String,
        data: Vec<u8>,
    }

    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}

fn rust_tuple_strunct() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn rust_enum() {
    #[derive(Debug)]
    enum PokerCard {
        Clubs(u8),
        Spades(u8),
        Diamonds(char),
        Hearts(char),
    }

    let c0 = PokerCard::Clubs(1);
    let c1 = PokerCard::Spades(5);
    let c2 = PokerCard::Diamonds('A');
    let c3 = PokerCard::Hearts('K');

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 1 };
    let m4 = Message::Write("aaa".to_string());
    let m3 = Message::ChangeColor(255, 255, 0);

    let some_num = Some(5);
    let some_string = Some("HHH".to_string());

    let none: Option<i32> = None;
}

fn rust_array() {
    let a = [1, 2, 3, 4, 5, 6];
    println!("请输入数组下标");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("输入结束");

    let index: usize = index.trim().parse().expect("数组下标不为数字");

    let ele = a[index];

    println!("下标内容为{}", ele)
}

fn rust_match() {
    enum Action {
        Say(String),
        MoveTo(i32, i32),
        ChangeColorRGB(u16, u16, u16),
        None,
    }

    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 0, 0),
        Action::None,
    ];

    let _move = actions.iter().filter(|a| match a {
        Action::MoveTo(_, _) => true,
        _ => false,
    });

    let __move = actions.iter().filter(|a| matches!(a, Action::MoveTo(_, _)));

    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("MoveTo val {},{}", x, y)
            }
            _ => (),
        }
    }
}

fn rust_myenum() {
    enum MyEnum {
        Foo,
        Bar,
    }

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];

    // v.iter().filter(|x| x == MyEnum::Foo);
    let list = v.iter().filter(|x| matches!(x, MyEnum::Foo));
}

fn rust_if_let() {
    let age = Some(60);
    println!("在匹配前，age是{:?}", age);
    if let Some(age) = age {
        let age = age * 2;
        println!("if let 匹配出来的age是{}", age);
    }

    println!("在匹配后，age是{:?}", age);

    match age {
        Some(age) => {
            let age = 8 * age;
            println!("match 匹配出来的age是{}", age);
        }
        _ => (),
    }

    println!("在匹配后，age是{:?}", age);
}

fn rust_option_plusone() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            None => None,
        }
    }
    let five = 5;
    let six = plus_one(Some(five));
    let none = plus_one(None);

    println!("{} | {:?} | {:?}", five, six, none);
}

fn rust_some_while() {
    let mut stack: Vec<i32> = Vec::new();
    const MAX: i32 = 5;
    while stack.len().to_i32() < Some(MAX) {
        if let Some(len) = stack.len().to_i32() {
            stack.push(len + 1);
        }
    }

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn rust_some_for() {
    let mut stack: Vec<i32> = Vec::new();
    for i in 1..5 {
        stack.push(i);
    }

    for (k, v) in stack.iter().enumerate() {
        println!("key:{} | value: {}", k, v);
    }
}

fn rust_struct_match() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 99 };

    match p {
        Point { x, y: 0 } => {
            println!("aaaa")
        }
        Point { x, y } => {
            println!("CCCCCCC")
        }
    }
}

fn rust_enum_match() {
    enum Test {
        Clone,
        Open(String, i32),
    }

    let test = Test::Open(String::from("aaaa"), 123);
    match test {
        Test::Open(target, id) => {
            println!("open target: {}, id: {}", target, id);
        }
        Test::Clone => {
            println!("Test Clone");
        }
        _ => (),
    }
}

fn rust_enum_match2() {
    enum Color {
        RGB(i32, i32, i32),
        HSV(i32, i32, i32),
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::RGB(255, 255, 9));

    match msg {
        Message::ChangeColor(Color::RGB(r, g, b)) => {
            println!("Color {}, {}, {}", r, g, b)
        }
        _ => (),
    }
}

fn rust_undresline() {
    let s = Some(String::from("Hello"));

    // Rust 中 「 _[变量名] 」等价于 「 [变量名] 」,作用是告诉编译器此变量不会被使用但是任然有用
    // if let Some(_s) = s {
    //     println!("ffff")
    // }

    if let Some(_) = s {
        println!("ffff")
    }

    println!("aaaa {:?}", s);
}

fn rust_match_guard() {
    let mut input_val = String::new();
    println!("请输入数字");
    io::stdin().read_line(&mut input_val).expect("输入结束");
    let num: Option<i32> = Some(input_val.trim().parse().expect("输入的内容无法解析为数字"));

    if let Some(num) = num {
        match num {
            x if x > 15 => {
                println!("x > 15")
            }
            1..=100 if num < 10 => println!("enum X, {}", num),
            x => println!("X Val: {}", x),
        }
    }
}

fn rust_at_match() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 40 };

    match msg {
        // 绑定运算，将一个字段绑定至另一个变量
        // @ 后可接 “范围选择”
        Message::Hello { id: id_val @ 1..=9 } => {
            println!("id_val val {}", id_val)
        }
        Message::Hello { id: 10..=50 } => {
            println!("id 10 > val < 50")
        }
        // 变量名可与字段名一样
        Message::Hello { id } => {
            println!("当前变量 id 值 {}", id);
        }
    }

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

    let point = Point { x: 1, y: 5 };
    // 通过Point解构进行判断，只有「 x 」字段为 10 时才为 true
    if let p @ Point { x: 10, y } = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }
}

/// @新特性(Rust 1.53 新增)
fn rust_at_new() {
    match 1 {
        // num @ 1 | 2 => {
        //     println!("{}", num);
        // }
        _ => {}
    }
}


fn rust_struct_impl() {
    #[derive(Debug)]
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }
    impl Circle {
        fn new() -> Circle {
            Circle {
                x: 1.0,
                y: 2.2,
                radius: 1.666,
            }
        }

        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    println!("{:?}", Circle::new());
    // return Circle::new() ;
}

fn rust_struct_pub () {
    #[derive(Debug)]
    pub struct Rectangle {
        width: u32,
        height: u32
    }

    impl Rectangle {
        pub fn new() -> Self {
            Rectangle { width: 32, height: 32 }
        }

        pub fn width (&self) -> u32 {
            return self.width;
        }
    }

    let r = Rectangle::new();
    println!("r vla {:?}", r.width());
}


fn main() {
    rust_struct_pub();
    // rust_struct_impl();
    // rust_at_match();
    // rust_match_guard();
    // rust_enum_match2();
    // rust_enum_match();
    // rust_struct_match();
    // rust_some_for();
    // rust_some_while();
    // rust_option_plusone();
    // rust_if_let();
    // rust_match();
    // rust_array();
    // rust_enum()
    // rust_tuple_strunct()
    // rust_struct_monney()
    // rust_new_struct()
    // rust_struct()
    // rust_string()
    // rust_course_8()
    // rust_str_slice()
    // rust_tag_fn()
    // rust_dangling_references_check()
    // rust_course_7()
    // println!("Hello, world!");

    // rust_course_1();
    // rust_course_2();
    // rust_course_3();
    // println!("const MAX_POINTS val {:?}", MAX_POINTS);
    // rust_course_4();
    // rust_course_5();
    // rust_float_num();
    // rust_nan_num()
    // rust_range();
    // rust_char();
    // rust_course_6();
    // rust_clone();
    // rust_shallow_copy();
    // rust_copy_move();
}
