use boring_derive::Builder;

#[derive(Debug, Default, Builder)]
struct Example {
    field1: f32,
    #[builder(no_into)]
    field2: String,
}

fn main() {
    let example = Example::default().field1(1.0).field2("something");
    println!("{:?}", example);
}
