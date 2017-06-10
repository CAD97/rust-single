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
[ ![issues closed](https://img.shields.io/github/issues-closed/CAD97/rust-single.svg)
](https://github.com/CAD97/rust-single/issues?q=is%3Aissue%20is%3Aclosed)

This crate provides the `Single` trait for extracting the element from a single-element iterator.

## License

You may use this crate under the MIT license or the Apache License 2.0 at your discretion.
This crate is dual-licensed for compatibility with rust itself.

## Trait single::Single

```rust
pub trait Single {
    fn single(self) -> Result<Self::Item, Error>;
}
```

### Required Methods

<dl>
  <dt><code>fn single(self) -> Result&lt;Self::Item, Error&gt;</code>
  <dd>
    <p>Get the single element from a single-element iterator.
    <h4>Examples</h4>
    <pre>
assert_eq!(iter::empty::<i32>().single(), Err(single::Error::NoElements));
assert_eq!(iter::once(0).single(), Ok(0));
assert_eq!(iter::repeat(0).single(), Err(single::Error::MultipleElements));
</dl>

### Implementors

```rust
impl<I: Iterator> Single for I {}
```
