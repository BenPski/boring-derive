use boring_derive::Builder;

#[derive(Debug, Default, Builder)]
#[builder(suffix = "_set")]
struct Example {
    item: String,
}

fn main() {
    let ex = Example::default().item_set("balls");
    println!("{:?}", ex);
}
