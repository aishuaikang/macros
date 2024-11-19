use macros::EnumFrom;

#[derive(Debug, EnumFrom)]
#[allow(unused)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Left(u32),
    Down,
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}

impl<T> DirectionUp<T> {
    fn new(speed: T) -> Self {
        Self { speed }
    }
}

fn main() {
    let up = Into::<Direction<i32>>::into(DirectionUp::new(42));
    let le = Into::<Direction<i32>>::into(33);

    println!("{:?}", up);
    println!("{:?}", le);
}
