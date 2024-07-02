use boring_derive::From;

#[derive(Debug, From)]
struct Example(usize);

fn main() {
    let ex: Example = 1.into();
    println!("{:?}", ex);
}
