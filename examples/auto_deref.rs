use macros::AutoDeref;

#[derive(AutoDeref)]
#[deref(mutable = true)]
pub struct BulkString {
    inner: String,
}

fn main() {
    let bulk_string = BulkString {
        inner: "hello".to_string(),
    };

    assert_eq!(&bulk_string.inner, &*bulk_string);
}
