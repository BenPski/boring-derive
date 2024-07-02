use boring_derive::From;

#[derive(Debug, From)]
enum Example {
    Nothing,
    #[from(not_real)]
    Number(f32),
    Str(String),
}

fn main() {}
