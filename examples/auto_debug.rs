use macros::AutoDebug;

#[allow(dead_code)]
#[derive(AutoDebug)]
#[debug(skip)]
pub struct BulkString {
    inner: String,
}

fn main() {
    let bulk_string = BulkString {
        inner: "hello".to_string(),
    };

    println!("{:?}", bulk_string);
}
