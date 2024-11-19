use macros::EnumFrom;

#[derive(Debug, EnumFrom)]
#[allow(unused)]
enum Direction {
    Up(DirectionUp),
    Left(u32),
    Down,
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: u32,
}

impl DirectionUp {
    fn new(speed: u32) -> Self {
        Self { speed }
    }
}

fn main() {
    let up = Into::<Direction>::into(DirectionUp::new(42));
    let le = Into::<Direction>::into(33);

    println!("{:?}", up);
    println!("{:?}", le);
}
