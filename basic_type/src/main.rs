/* 
 *   基本类型：
 *  Rust 每个值都有其确切的数据类型，总的来说可以分为两类：基本类型和复合类型。 基本类型意味着它们往往是一个最小化原子类型，无法解构为其它类型(一般意义上来说)，由以下组成：
 *
 *   数值类型: 有符号整数 (i8, i16, i32, i64, isize)、 无符号整数 (u8, u16, u32, u64, usize) 、浮点数 (f32, f64)、以及有理数、复数
 *   字符串：字符串字面量和字符串切片 &str
 *   布尔类型： true和false
 *   字符类型: 表示单个 Unicode 字符，存储为 4 个字节
 *   单元类型: 即 () ，其唯一的值也是 ()
 */
fn main() {
    //let guess = "42".parse().expect("Not a number!");

    //浮点数精度
    /* 
    let abc:(f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz:(f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 == xyz.2);    
    */
    
    //数字运算
    /* 
    let twenty = 20;
    let twenty_one:i32 = 21;
    let twenty_two = 22i32;
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million : i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_two = [
        42.0,
        42f32,
        42.0f32
    ];

    println!("{:.2}", forty_two[0]);   
    */

    //位运算
    /* 
    let a: i32 = 2;
    let b: i32 = 3;

    println!("a & b = {}", a & b);
    println!("a | b = {}", a | b);
    println!("a ^ b = {}", a ^ b);
    println!("!b = {}", !b);
    println!("(a << b) = {}", a << b);
    println!("(a >> b) = {}", a >> b);

    let mut a = a;
    a <<= b;
    println!("a <<=b == {}", a);    
    */

    //序列
    /* 
    for i in 1..=5 {
        println!("{}", i);
    }

    for i in 'a'..='z' {
        println!("{}", i);
    }   
    */

    //字符类型
    /* 
    let c = '中';
    println!("字符‘中’占用内存大小为{}",std::mem::size_of_val(&c));    
    */

    //bool类型
    /* 
    let _t: bool = true;
    let f = false;
    if f {
        println!("这是一段毫无意义的代码");
    }    
    */

    //单元类型
    /* 
    单元类型就是 () ，对，你没看错，就是 () ，唯一的值也是 ()
    main 函数就返回这个单元类型 ()，你不能说 main 函数无返回值，因为没有返回值的函数在 Rust 中是有单独的定义的：发散函数( diverge function )，顾名思义，无法收敛的函数。
    例如常见的 println!() 的返回值也是单元类型 ()。
    再比如，你可以用 () 作为 map 的值，表示我们不关注具体的值，只关注 key。
    */
    

}
