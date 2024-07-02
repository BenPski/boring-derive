use boring_derive::From;

#[derive(Debug, From)]
struct Example;

fn main() {
    let ex: Example = ().into();
    println!("{:?}", ex);
}
