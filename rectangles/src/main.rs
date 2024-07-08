#[derive(Debug)]
struct Rectangles {
    height:u32,
    width:u32,
}
impl Rectangles {
    fn area(&self) -> u32{
        self.height * self.width
    }

    fn width(&self) -> bool{
        self.width > 0
    }

    fn can_hold(&self, rec: &Rectangles) -> bool
    {
        self.width >= rec.width && self.height >= rec.height
    }

    fn square(size: u32) -> Self{
        Self{
            width:size,
            height:size,
        }
    }
}
fn main() {
    let rect1 = Rectangles{
        width : 30,
        height : 50,
    };
    let rect2 = Rectangles{
        width : 40,
        height : 40,
    };
    let rect3 = Rectangles{
        width : 20,
        height : 40,
    };
    dbg!(&rect1);
    println!("{:#?}",rect1); 
    println!("The area of the rectangle is {} square pixels.", rect1.area());    
    if rect1.width() {
        println!("The rectangle is nonezero");
    }
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangles::square(4);
    dbg!(rect4);
}

/* fn area(rect: &Rectangles) -> u32 {
    rect.width * rect.height
} */
