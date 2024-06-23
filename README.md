# boring-derive

Derive macros for simple implementations of traits.

For example `From` usually has very trivial implementations.
```rust
enum Thing<A,B,C> {
  Item1(A),
  Item2(B),
  Item3(C),
}

impl<A,B,C> From<A> for Thing<A,B,C> {
  fn from(value: A) -> Self {
    Thing::Item1(value)
  }
}

impl<A,B,C> From<B> for Thing<A,B,C> {
  fn from(value: B) -> Self {
    Thing::Item2(value)
  }
}

impl<A,B,C> From<C> for Thing<A,B,C> {
  fn from(value: C) -> Self {
    Thing::Item3(value)
  }
}
```

So instead just:
```rust
#[derive(From)]
enum Thing<A,B,C> {
  Item1(A),
  Item2(B),
  Item3(C),
}
```
