use std::collections::HashMap;

mod comp;

fn greet_word() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello!";
    let regions = [southern_germany, chinese, english];
    for region in regions {
        println!("{}", &region);
    }
}

fn sample() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";
    let records = penguin_data.lines();
    for (i, record) in records.enumerate(){
        if i==0 || record.trim().len() == 0 {
            continue;
        }

        let fields:Vec<_> = record.split(",").map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields)
        }

        let name=fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length)
        }
    }
}

struct S1 {
    e: i32
}

const _MAX_POINTS: u32 = 100_000;

fn variables_test() {
    //let x = 5;  unmutable will cause failure
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // unused variable, add "_" as variable prefix
    // wrong:  let a=10
    let _a=10;
    let _b=10;

    // de-construct
    let (a,b,c,d,e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1,2,3,4,5];
    S1 {e, ..} = S1 {e:5};
    assert_eq!([1,2,1,4,5], [a,b,c,d,e]);

    //shadowing
    let t=5;
    // 在variable_test函数的作用域内对之前的x进行遮蔽
    let t=t+1;
    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let t = t*2;
        println!("The value of t in the inner scope is: {}", t);
    }
    println!("The value of t is: {}", t);
}

fn types_learn() {
    // wrong: let guess = "42".parse().expect("Not a number!");
    let _guess: i32 = "42".parse().expect("Not a number");
    let _guess = "42".parse::<i32>().expect("Not a number");
    let d=98_222;
    println!("{}", d);

    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为")
    }

    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;

    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];

    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);
    

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 != xyz.2);
}

fn range_learn() {
    for i in 1..5 {
        println!("{}",i);
    }
    // add "=" means including the last element
    for i in 1..=5 {
        println!("{}",i);
    }
    for i in 'a'..='z' {
        println!("{}",i);
    }
}

fn hashmap_unit() {
    let mut m = HashMap::new();
    m.insert("test1", ());
    m.insert("test2", ());
    println!("map: {:?}", m);
}

fn char_learn() {
    let c = 'A';
    let d = '中';
    println!("char {} take {} byte memory", c, std::mem::size_of_val(&c));
    println!("char {} take {} byte memory", d, std::mem::size_of_val(&d));
}

#[allow(unused_assignments)]
fn string_learn() {
    let mut s1 = "hello";
    let mut s2= String::from("demo");
    s2.push_str(", jimmy");
    s1 = "ahaha";
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn reference_learn() {
    //同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
    //引用必须总是有效的

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);
    // 新编译器中，r3作用域在这里结束
}

fn ownsership_learn() {
    {
        //Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
        //一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
        //当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)
        let s = String::from("hello");  // s 进入作用域

        takes_ownership(s);             // s 的值移动到函数里 ...
                                        // ... 所以到这里不再有效

        let x = 5;                      // x 进入作用域

        makes_copy(x);                  // x 应该移动函数里，但 i32 是 Copy 的，所以在后面可继续使用 x

        println!("ownership-{}", x)

    } // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
    // 所以不会有特殊操作

    fn takes_ownership(some_string: String) { // some_string 进入作用域
        println!("ownership-{}", some_string);
    } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

    fn makes_copy(some_integer: i32) { // some_integer 进入作用域
        println!("ownership-{}", some_integer);
    } // 这里，some_integer 移出作用域。不会有特殊操作


    {
        let _s1 = gives_ownership();         // gives_ownership 将返回值
                                            // 移给 s1
    
        let s2 = String::from("hello");     // s2 进入作用域
    
        let _s3 = takes_and_gives_back(s2);  // s2 被移动到
                                            // takes_and_gives_back 中,
                                            // 它也将返回值移给 s3
    } // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
      // 所以什么也不会发生。s1 移出作用域并被丢弃
    
    fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
                                                 // 调用它的函数
    
        let some_string = String::from("hello"); // some_string 进入作用域.
    
        some_string                              // 返回 some_string 并移出给调用的函数
    }
    
    // takes_and_gives_back 将传入字符串并返回该值
    fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    
        a_string  // 返回 a_string 并移出给调用的函数
    }
}

fn main() {
    // println!("Hello, world!");
    greet_word();
    sample();
    variables_test();
    types_learn();
    range_learn();
    comp::complex_learn();
    char_learn();
    hashmap_unit();
    string_learn();
    reference_learn();
    ownsership_learn();
}
