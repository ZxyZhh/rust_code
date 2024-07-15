fn main() {
    // let v:Vec<i32> = Vec::new();
    // let v = vec![1,2,3,4,5,6,7];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    let v1 = vec![1,2,3,4,5,6,7];
    let third : &i32 = &v1[2];
    println!("The third element is {third}");

    let third : Option<&i32> = v1.get(2);
    match third {
        Some(third) => println!("The third elementis {third}"),
        None => println!("There is no third element"),

    }

    let does_not_exist = v1.get(100);

    match does_not_exist {
        Some(fourth) => println!("The 100th element is {fourth}"),
        None => println!("There is no 100th element"),
    }


    let first = &v1[0];
    v.push(6);
    println!("The first element is: {}", first);

    for i in &v1 {
        println!("{}", i);
    }

    for i in &mut v{
        *i += 1;
    }

    for i in &v{
        println!("{}", i);
    }
    
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3), 
        SpreadSheetCell::Float(10.12), 
        SpreadSheetCell::Text(String::from("blue"))
        ];
    for i in &row{
        match i{
            SpreadSheetCell::Int(value) => println!("value = {value}"),
            SpreadSheetCell::Float(value) => println!("value = {value}"),
            SpreadSheetCell::Text(value) => println!("value = {value}"),
        }
    }



}
