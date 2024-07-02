use boring_derive::From;

#[derive(Debug, From)]
struct Example(usize, f32);

fn main() {
    let ex: Example = (1, 1.0).into();
    println!("{:?}", ex);
}
