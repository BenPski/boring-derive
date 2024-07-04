use boring_derive::From;

#[derive(Debug, From)]
enum Example {
    Nothing,
    #[from(skip)]
    Number(f32),
    Str(String),
}

fn main() {
    let ex: Example = 1.3.into();
    println!("{:?}", ex);
}
