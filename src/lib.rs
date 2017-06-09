//! This crate exposes a `Single` trait for extracting the element from a
//! single-element iterator (or `panic!`ing if that precondition is false).

use std::{fmt, error, result};

type Result<T> = result::Result<T, self::Error>;

/// Trait to extract the element from a single-element iterator.
pub trait Single {
    /// The item type of the wrapped iterator.
    type Item;

    /// Get the single element from a single-element iterator.
    ///
    /// Note that many iterators return references to the elements,
    /// so this method will as well if the backing iterator does.
    ///
    /// # Examples
    ///
    /// ```
    /// # use single::{ self, Single };
    /// # use std::iter;
    /// assert_eq!(iter::empty::<i32>().single(), Err(single::Error::NoElements));
    /// assert_eq!(iter::once(0).single(), Ok(0));
    /// assert_eq!(iter::repeat(0).single(), Err(single::Error::MultipleElements));
    /// ```
    fn single(self) -> Result<Self::Item>;
}

/// An error in the execution of [`single::Single::single`](trait.Single.html#tymethod.single).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Asked empty iterator for single element.
    NoElements,
    /// Asked iterator with multiple elements for single element.
    MultipleElements,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NoElements => write!(
                f,
                "SingleError::NoElements: {}",
                error::Error::description(self),
            ),
            Error::MultipleElements => write!(
                f,
                "SingleError::MultipleElements: {}",
                error::Error::description(self),
            ),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NoElements => "Asked empty iterator for single element",
            Error::MultipleElements => "Asked iterator with multiple elements for single element",
        }
    }
    fn cause(&self) -> Option<&error::Error> { None }
}

impl<I> Single for I where I: Iterator {
    type Item = <Self as Iterator>::Item;

    fn single(mut self) -> Result<Self::Item> {
        match self.next() {
            None => Err(Error::NoElements),
            Some(element) => if self.next().is_none() {
                Ok(element)
            } else {
                Err(Error::MultipleElements)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::iter;
    use super::Single;

    #[test]
    #[should_panic(expected = "Asked empty iterator for single element")]
    fn panic_empty() { let _: i32 = iter::empty().single().unwrap(); }

    #[test]
    #[should_panic(expected = "Asked iterator with multiple elements for single element")]
    fn panic_multiple() { let _ = iter::repeat(0).single().unwrap(); }
}
