fn main(){
/*     let mut s = String::new();
    let data = "initial content";
    s = data.to_string();
    println!("s = {}", s); */
    
    // let s = String::from("initial content");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s = {}", s);
    s.push('l');
    println!("s = {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toc");
    let s4 = format!("{s1}-{s2}-{s3}");
    println!("s4 = {}", s4);

    let s5 = String::from("hello");
    let h = s5[0];
    println!("The first character is {}", h);
}