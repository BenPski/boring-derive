use std::usize;

use boring_derive_macro::From;

// #[derive(Debug, From)]
// struct Example {
//     item: String,
// }
//
// #[derive(Debug, From)]
// struct Thing {}
//
// #[derive(Debug, From)]
// struct Example2 {
//     item: String,
//     another: usize,
// }
//
// #[derive(Debug, From)]
// struct Stuff<T> {
//     item: T,
// }
//
// #[derive(Debug, From)]
// struct A;
//
// #[derive(Debug, From)]
// struct B(usize);
//
// #[derive(Debug, From)]
// struct C(String, usize);

#[derive(Debug, From)]
enum Balls {
    Left(String),
    Right(usize),
}

fn main() {
    // let x = String::from("balls");
    // let e: Example = x.into();
    // println!("{:?}", e);
    //
    // let x = ();
    // let e: Thing = x.into();
    // println!("{:?}", e);
    //
    // let y = (String::from("weiner"), 2);
    // let e: Example2 = y.into();
    // println!("{:?}", e);
    //
    // let x = 3;
    // let e: Stuff<usize> = x.into();
    // println!("{:?}", e);
    //
    // let x = ();
    // let e: A = x.into();
    // println!("{:?}", e);
    //
    // let x = 3;
    // let e: B = x.into();
    // println!("{:?}", e);
    //
    // let x = (String::from("dong"), 3);
    // let e: C = x.into();
    // println!("{:?}", e);
    //
    let x = String::from("dong");
    let e: Balls = x.into();
    println!("{:?}", e);
}
