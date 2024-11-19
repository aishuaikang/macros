fn main() -> anyhow::Result<()> {
    let v: Vec<i32> = my_vec!["1".parse()?, "2".parse()?, "3".parse()?,];
    println!("{:?}", v);
    let v = vec![1, 2, 3, 4, 5];
    println!("{:?}", v);

    <[i32]>::into_vec(Box::new([1, 2, 3, 4, 5]));
    // let s: Box<[i32]> = Box::new([10, 40, 30]);
    // let x = s.into_vec();
    Ok(())
}

#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };
    ($($i:expr),+ $(,)?) => {
        <[_]>::into_vec(Box::new([$($i),+]))

        // {
        //     let mut temp_vec = Vec::new();
        //     $(
        //         temp_vec.push($i);
        //     )*
        //     temp_vec
        // }
    };
}
