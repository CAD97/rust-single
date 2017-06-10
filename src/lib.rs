//! Provides the `Single` trait for extracting the element from a
//! single-element iterator.
//!
//! # Examples
//!
//! ```
//! use single::{ Single, Error };
//! use std::iter;
//!
//! assert_eq!(iter::empty::<i32>().single(), Err(Error::NoElements));
//! assert_eq!(iter::once(0).single(), Ok(0));
//! assert_eq!(iter::repeat(0).single(), Err(Error::MultipleElements));
//! ```

#[macro_use]
extern crate quick_error;

use std::{error, fmt, result};

type Result<T> = result::Result<T, Error>;

pub trait Single: Iterator {
    /// Get the single element from a single-element iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use single::{ Single, Error };
    /// # use std::iter;
    /// assert_eq!(iter::empty::<i32>().single(), Err(Error::NoElements));
    /// assert_eq!(iter::once(0).single(), Ok(0));
    /// assert_eq!(iter::repeat(0).single(), Err(Error::MultipleElements));
    /// ```
    fn single(self) -> Result<Self::Item>;
}

quick_error! {
    /// An error in the execution of
    /// [`Single::single`](trait.Single.html#tymethod.single).
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Error {
        /// Asked empty iterator for single element.
        NoElements {
            description("Called single() on empty iterator")
        }
        /// Asked iterator with multiple elements for single element.
        MultipleElements {
            description("Called single() on multiple-element iterator")
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))
    }
}

impl<I: Iterator> Single for I {
    fn single(mut self) -> Result<Self::Item> {
        match self.next() {
            None => Err(Error::NoElements),
            Some(element) => {
                match self.next() {
                    None => Ok(element),
                    Some(_) => Err(Error::MultipleElements),
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::Single;
    use std::iter;

    #[test]
    #[should_panic(expected = "Called single() on empty iterator")]
    fn panic_empty() { let _: i32 = iter::empty().single().unwrap(); }

    #[test]
    #[should_panic(expected = "Called single() on multiple-element iterator")]
    fn panic_multiple() { let _ = iter::repeat(0).single().unwrap(); }
}
