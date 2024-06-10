use macros::EnumFromDarling;

#[allow(unused)]
#[derive(Debug, EnumFromDarling)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(u32),
    Right,
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}

fn main() {
    let up: Direction<u32> = DirectionUp::new(10).into();
    let left: Direction<u32> = 10.into();
    println!("{:?}, {:?}", up, left);
}

impl<T> DirectionUp<T> {
    fn new(speed: T) -> Self {
        Self { speed }
    }
}

// impl<T> From<DirectionUp<T>> for Direction<T> {
//     fn from(v: DirectionUp<T>) -> Self {
//         Direction::Up(v)
//     }
// }
