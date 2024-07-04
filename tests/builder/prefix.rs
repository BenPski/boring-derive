use boring_derive::Builder;

#[derive(Debug, Default, Builder)]
#[builder(prefix = "set_")]
struct Example {
    item: String,
}

fn main() {
    let ex = Example::default().set_item("balls");
    println!("{:?}", ex);
}
