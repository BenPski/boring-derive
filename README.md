# boring-derive

Derive macros for simple implementations of traits.

For example `From` usually has very trivial implementations.
```rust
enum Thing {
  Item1(String),
  Item2(usize),
  Item3(f32),
}

impl From<String> for Thing {
  fn from(value: String) -> Self {
    Thing::Item1(value)
  }
}

impl From<usize> for Thing {
  fn from(value: usize) -> Self {
    Thing::Item2(value)
  }
}

impl From<f32> for Thing {
  fn from(value: f32) -> Self {
    Thing::Item3(value)
  }
}
```

So instead just:
```rust
#[derive(From)]
enum Thing {
  Item1(String),
  Item2(usize),
  Item3(f32),
}
```
