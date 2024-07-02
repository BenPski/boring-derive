use boring_derive::Builder;

#[derive(Debug, Default, Builder)]
struct Example {
    #[builder(skip)]
    field1: f32,
    field2: String,
}

fn main() {
    let example = Example::default().field1(0.0).field2("something");
    println!("{:?}", example);
}
