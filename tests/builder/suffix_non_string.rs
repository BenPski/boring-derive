use boring_derive::Builder;

#[derive(Debug, Default, Builder)]
#[builder(suffix = 1)]
struct Example {
    item: String,
}

fn main() {
    let ex = Example::default();
    println!("{:?}", ex);
}
