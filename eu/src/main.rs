#[derive(Debug)]
enum IpAddrKind{
    V4(u8),
    V6{x:u8, y:u8},
}
fn main() {
    let four = IpAddrKind::V4(4);
    match four {
        IpAddrKind::V4(i) => println!("i = {}", i),
        IpAddrKind::V6{x,y} => println!("x = {}, y = {}", x, y),
    }
}
