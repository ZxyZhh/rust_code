#![allow(unused_variables)]

fn main() {
    let s = "hello world";
    // greet(s);
    let hello = &s[0..5];
    let world = &s[6..11];
    greet1(s);
    greet1(hello);
    greet1(world);

    let mut r = String::from("Hello ");
    r.push_str("rust");
    println!("r = {}", r);
    r.push('!');
    println!("r = {}", r);

    let mut x = String::from("Hello rust!");
    x.insert(5, ',');
    println!("x = {}", x);
    x.insert_str(6, "I like");
    println!("x = {}", x);

    let mut string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string = string_replace.replace("rust", "RUST");
    println!("string_replace = {}", string_replace);
    println!("new_string = {}", new_string)
}

#[allow(dead_code)]
fn greet(name: String) {
    println!("name = {}", name);
}

fn greet1(name: &str) {
    println!("name = {}", name);
}