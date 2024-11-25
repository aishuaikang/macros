use macros::AutoDebug;

#[allow(dead_code)]
#[derive(AutoDebug)]
pub struct BulkString {
    // #[debug(skip)]
    inner: String,
    #[debug(skip)]
    nothing: (),
}

fn main() {
    let bulk_string = BulkString {
        inner: "hello".to_string(),
        nothing: (),
    };

    println!("{:?}", bulk_string);
}
