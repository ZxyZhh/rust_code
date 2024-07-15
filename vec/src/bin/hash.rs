use std::collections::HashMap;
fn main(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Black");
    let blue = scores.get(&team_name).copied().unwrap_or(0);
    println!("bule team score: {}", blue);

    for (key, value) in &scores{
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 20);
    println!("bule team score: {}",scores.get("Blue").copied().unwrap_or(0));

    scores.entry(String::from("Blue")).or_insert(10);
    scores.entry(String::from("Black")).or_insert(100);
    for (key, value) in &scores{
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    for (key, value) in &map{
        println!("{}: {}", key, value);
    }

}