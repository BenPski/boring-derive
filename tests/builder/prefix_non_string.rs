use boring_derive::Builder;

#[derive(Debug, Default, Builder)]
#[builder(prefix = 1)]
struct Example {
    item: String,
}

fn main() {
    let ex = Example::default().set_item("balls");
    println!("{:?}", ex);
}
