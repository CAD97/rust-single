# Single

This crate exposes a `Single` trait for extracting the element from a 
single-element iterator (or `panic!`ing if that precondition is false).

## License

You may use this crate under the MIT license or the Apache License (Version 2)
at your discretion. This crate is dual-licensed for compatibility with rust itself.

## Trait single::Single

```rust
pub trait Single {
    type Item;
    fn single(self) -> Self::Item;
    fn single_or(self, default: Self::Item) -> Self::Item;
    fn single_or_else<F>(self, default_fn: F) -> Self::Item
        where F: FnOnce() -> Self::Item;
}
```

### Associated Types

<dl>
  <dt><code>type Item</code>
  <dd><p>The item type of the wrapped iterator.
</dl>

### Required Methods

<dl>
  <dt><code>fn single(self) -> Self::Item</code>
  <dd>
    <p>Get the single element from a single-element iterator.
    <p>Note that many iterators return references to the elements, so this method will as well if the backing iterator does.
    <h4>Panics</h4>
    <p>Panics if the iterator is empty or has more than one element.
    <h4>Examples</h4>
    <p>An emtpy iterator panics:
    <pre>iter::empty().single();</pre>
    <p>An iterator with multiple elements panics:
    <pre>iter::repeat(0).single();</pre>
    <p>An iterator with a single element returns that element:
    <pre>assert_eq!(iter::once(0).single(), 0);</pre>
  <dt><code>fn single_or(self, default: Self::Item) -> Self::Item</code>
  <dd>
    <p>Get the single element from an iterator or a default fallback value.
    <p>Note that many iterators return references to the elements, so this method will as well if the backing iterator does.
    <h4>Examples</h4>
    <p>An empty iterator will return the fallback:
    <pre>assert_eq!(iter::empty().single_or(5), 5)</pre>
    <p>An iterator with multiple elements will return the fallback:
    <pre>assert_eq!(iter::repeat(0).single_or(5), 5)</pre>
    <p>An iterator with a single element returns that element:
    <pre>assert_eq!(iter::once(0).single_or(5), 0)</pre>
  <dt><code>fn single_or_else<F>(self, default_fn: F) -> Self::Item where F: FnOnce() -> Self::Item</code>
  <dd>
    <p>Get the single element from an iterator or from a default provider.
    <p>Note that many iterators return references to the elements, so this method will as well if the backing iterator does.
    <h4>Examples</h4>
    <p>An empty iterator will return the fallback:
    <pre>assert_eq!(iter::empty().single_or_else(|| 5), 5)</pre>
    <p>An iterator with multiple elements will return the fallback:
    <pre>assert_eq!(iter::repeat(0).single_or_else(|| 5), 5)</pre>
    <p>An iterator with a single element returns that element:
    <pre>assert_eq!(iter::once(0).single_or_else(|| 5), 0)</pre>
</dl>

### Implementors

```rust
impl<I> Single for I
  where I: Iterator
```
