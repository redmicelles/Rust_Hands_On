#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32
}

//methods defined for Rectangle struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && 
        self.height > other.height
    }
}

fn main() {
   let rect1 = Rectangle {
        width: 30,
        height: 50
    };

   let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

   let rect3 = Rectangle {
        width: 60,
        height: 45
    };

   if rect1.width() {
            println!("The rectangle has a nonzero width; it is {}", rect1.width);
        }
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    //Debug print
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    //check if rect1 can contain any other rectangle
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
