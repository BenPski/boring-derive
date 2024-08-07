use boring_derive::From;

#[derive(Debug, From)]
enum Example {
    Nothing,
    Number(f32),
    Str(String),
}

fn main() {
    let ex: Example = ().into();
    println!("{:?}", ex);

    let ex: Example = 1.3.into();
    println!("{:?}", ex);

    let ex: Example = "something".to_string().into();
    println!("{:?}", ex);
}
