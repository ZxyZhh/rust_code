fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        6
    };

    println!("number = {}", number);

    let n = 6;
    if n % 4 == 0 {
        println!("Number is divisible by four");
    } else if n % 3 == 0 {
        println!("Number is divisible by three");
    } else if n % 2 == 0{
        println!("Number is divisible by two");
    }

    for i in 1..=5 {
        println!("{}", i);
    }

    let a = [4,3,2,1];
    for (i, v) in a.iter().enumerate() {
        println!("第 {} 个元素 = {}", i + 1, v);
    }

    for i in 1..4 {
        if i == 2 {
            continue;
        }
        println!("{}", i);
    }

    for i in 1..4 {
        if i == 2 {
            break;
        }
        println!("{}", i);
    }

    let mut n = 0;
    while n <= 5 {
        println!("while {}", n);
        n+=1;
    }
    println!("我出来了");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break 2*counter;
        }
    };
    println!("result = {}", result);
}
