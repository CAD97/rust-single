//! This crate exposes a `Single` trait for extracting the element from a
//! single-element iterator (or `panic!`ing if that precondition is false).

/// Trait to extract the element from a single-element iterator.
pub trait Single {
    /// The item type of the wrapped iterator.
    type Item;

    /// Get the single element from a single-element iterator.
    ///
    /// Note that many iterators return references to the elements,
    /// so this method will as well if the backing iterator does.
    ///
    /// # Panics
    ///
    /// Panics if the iterator is empty or has more than one element.
    ///
    /// # Examples
    ///
    /// An empty iterator panics:
    ///
    /// ```should_panic(expected = "Asked empty iterator for single element")
    /// # use single::Single;
    /// # use std::iter;
    /// # let _: i32 =
    /// iter::empty().single();
    /// ```
    ///
    /// An iterator with multiple elements panics:
    ///
    /// ```should_panic(expected = "Asked iterator with multiple elements for single element")
    /// # use single::Single;
    /// # use std::iter;
    /// iter::repeat(0).single();
    /// ```
    ///
    /// An iterator with a single element returns that element:
    ///
    /// ```
    /// # use single::Single;
    /// # use std::iter;
    /// assert_eq!(iter::once(0).single(), 0);
    /// ```
    fn single(self) -> Self::Item;

    /// Get the single element from an iterator or a default fallback value.
    ///
    /// Note that many iterators return references to the elements,
    /// so this method will as well if the backing iterator does.
    ///
    /// # Examples
    ///
    /// An empty iterator will return the fallback:
    ///
    /// ```
    /// # use single::Single;
    /// # use std::iter;
    /// assert_eq!(iter::empty().single_or(5), 5)
    /// ```
    ///
    /// An iterator with multiple elements will return the fallback:
    ///
    /// ```
    /// # use single::Single;
    /// # use std::iter;
    /// assert_eq!(iter::repeat(0).single_or(5), 5)
    /// ```
    ///
    /// An iterator with a single element returns that element:
    ///
    /// ```
    /// # use single::Single;
    /// # use std::iter;
    /// assert_eq!(iter::once(0).single_or(5), 0)
    /// ```
    fn single_or(self, default: Self::Item) -> Self::Item;

    /// Get the single element from an iterator or from a default provider.
    ///
    /// Note that many iterators return references to the elements,
    /// so this method will as well if the backing iterator does.
    ///
    /// # Examples
    ///
    /// An empty iterator will return the fallback:
    ///
    /// ```
    /// # use single::Single;
    /// # use std::iter;
    /// assert_eq!(iter::empty().single_or_else(|| 5), 5)
    /// ```
    ///
    /// An iterator with multiple elements will return the fallback:
    ///
    /// ```
    /// # use single::Single;
    /// # use std::iter;
    /// assert_eq!(iter::repeat(0).single_or_else(|| 5), 5)
    /// ```
    ///
    /// An iterator with a single element returns that element:
    ///
    /// ```
    /// # use single::Single;
    /// # use std::iter;
    /// assert_eq!(iter::once(0).single_or_else(|| 5), 0)
    /// ```
    fn single_or_else<F>(self, default_fn: F) -> Self::Item
        where F: FnOnce() -> Self::Item;
}

impl<I> Single for I where I: Iterator {
    type Item = <Self as Iterator>::Item;

    fn single(mut self) -> Self::Item {
        match self.next() {
            None => panic!("Asked empty iterator for single element"),
            Some(element) => if let Some(_) = self.next() {
                panic!("Asked iterator with multiple elements for single element")
            } else { element }
        }
    }

    fn single_or(self, default: Self::Item) -> Self::Item {
        self.single_or_else(|| default)
    }

    fn single_or_else<F>(mut self, default_fn: F) -> Self::Item
        where F: FnOnce() -> Self::Item
    {
        match self.next() {
            None => default_fn(),
            Some(element) => if let Some(_) = self.next() {
                default_fn()
            } else { element }
        }
    }
}


