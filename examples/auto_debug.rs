use macros::AutoDebug;

#[allow(unused)]
#[derive(AutoDebug)]
pub struct RespBulkString {
    inner: String,
    #[debug(skip)]
    nothing: (),
    hello: u32,
    world: String,
}

fn main() {
    let s = RespBulkString {
        inner: "hello".to_string(),
        nothing: (),
        hello: 42,
        world: "world".to_string(),
    };
    println!("{:?}", s);
}
