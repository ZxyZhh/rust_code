fn main() {
    another_function(1, 2);
    let x = plus_five(6);
    println!("x = {}", x);
}

fn another_function(i: i32, j: i32) {
    println!("i = {}", i);
    println!("j = {}", j);
}

fn plus_five(i: i32) -> i32 {
    if i > 5 {
        return i -5
    }
    i + 5
}

/* 
发散函数
fn dead_end() -> !{
    ...
}
*/