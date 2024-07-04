use boring_derive::Builder;

#[derive(Debug, Default, Builder)]
struct Example {
    #[builder(rename = "value")]
    item: String,
}

fn main() {
    let ex = Example::default().value("val");
    println!("{:?}", ex);
}
