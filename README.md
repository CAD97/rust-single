# This crate is deprecated.

Use [`Itertools::at_most_one`](https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.at_most_one) instead, which provides equivalent functionality in a better fashion.

-----

# Single
[ ![Build Status](https://img.shields.io/travis/CAD97/rust-single.svg)
](https://travis-ci.org/CAD97/rust-single)
[ ![crates.io](https://img.shields.io/crates/v/single.svg)
](https://crates.io/crates/single)
[ ![downloads](https://img.shields.io/crates/d/single.svg)
](https://crates.io/crates/single)
[ ![version downloads](https://img.shields.io/crates/dv/single.svg)
](https://crates.io/crates/single)
[ ![issues open](https://img.shields.io/github/issues/CAD97/rust-single.svg)
](https://github.com/CAD97/rust-single/issues)

This crate provides the `Single` trait for extracting the element from a single-element iterator.

## License

You may use this crate under the MIT license or the Apache License 2.0 at your discretion.

## Trait single::Single

```rust
pub trait Single: Iterator {
    fn single(self) -> Result<Self::Item, Error>;
}
```

### Required Methods

<dl>
  <dt><code>fn single(self) -> Result&lt;Self::Item, Error&gt;</code>
  <dd>
    <p>Get the single element from a single-element iterator.
    <h4>Examples</h4>

```rust
assert_eq!(iter::empty::<i32>().single(), Err(single::Error::NoElements));
assert_eq!(iter::once(0).single(), Ok(0));
assert_eq!(iter::repeat(0).single(), Err(single::Error::MultipleElements));
```
</dl>

### Implementors

```rust
impl<I: Iterator> Single for I {}
```
