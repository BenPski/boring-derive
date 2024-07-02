use boring_derive::Builder;

#[derive(Builder)]
struct Example {
    #[builder(not_real)]
    item: usize,
}

fn main() {}
