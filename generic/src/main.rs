fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let  result:&i32 = largest(&number_list);

/*     for number in &number_list{
        if number > largest{
            largest = number;
        }
    } */

    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let  result:&i32 = largest(&number_list);
/* 
    for number in &number_list{
        if number > largest{
            largest = number;
        }
    } */

    println!("The largest number is {result}");
}

fn largest<T>(list: &[T]) -> &i32{
    let mut largest = &list[0];

    for number in list{
        if number > largest{
            largest = number;
        }
    }

    largest
}

