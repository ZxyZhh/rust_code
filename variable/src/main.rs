fn main() {
    //1、变量不可变，mut可变
/*     let mut a:i32 = 5;
    println!("a = {}", a);
    a = 6; 
    println!("a = {}", a); */

    //2、下划线忽略未使用变量
/*     let _x = 5;
    let _y = 6; */

    //3、let解构，重新绑定
/*     let (a, mut b):(bool, bool) = (true, false);
    println!("a = {}, b = {}", a, b);

    b = true;
    println!("b = {}", b); */

    //4、变量和常量的差异
    /*变量的值不能更改可能让你想起其他另一个很多语言都有的编程概念：常量(constant)。与不可变变量一样，常量也是绑定到一个常量名且不允许更改的值，但是常量和变量之间存在一些差异：

    常量不允许使用 mut。常量不仅仅默认不可变，而且自始至终不可变，因为常量在编译完成后，已经确定它的值。
    常量使用 const 关键字而不是 let 关键字来声明，并且值的类型必须标注。 
    常量可以在任意作用域内声明，包括全局作用域，在声明的作用域内，常量在程序运行的整个过程中都有效。对于需要在多处代码共享一个不可变的值时非常有用，例如游戏中允许玩家赚取的最大点数或光速。
    例子：const MAX_POINTS: u32 = 100_000;
    */

    //6、变量遮蔽
    //这和 mut 变量的使用是不同的，第二个 let 生成了完全不同的新变量，两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配 ，而 mut 声明的变量，可以修改同一个内存地址上的值，并不会发生内存对象的再分配，性能要更好。
    let x = 5;
    let x = x+1;
    {
        let x = x*2;
        println!("x = {}", x);
    }
    println!("x = {}", x);

    

    
}
