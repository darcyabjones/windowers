//! An iterator for subslices of a size.
//!
//! See also: `std::slice::Windows`
//! This structure wasn't right for our purposes because we need the last window even if it's
//! imperfect.

use std::fmt;

#[derive(Debug, Clone)]
pub struct Windows<'a, T: 'a> {
    elements: &'a [T],
    size: usize,
    step: usize,
    start: usize,
}


impl<'a, T> Windows<'a, T> {

    /// Create a new windows object.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use windowrs::{Windows,Window};
    ///
    /// let mut win = Windows::new(&[1, 2, 3, 4], 2, 1);
    /// assert_eq!(win.next().unwrap(), Window::new(0, 2, &[1, 2][..]));
    /// assert_eq!(win.next().unwrap(), Window::new(1, 3, &[2, 3][..]));
    /// assert_eq!(win.next().unwrap(), Window::new(2, 4, &[3, 4][..]));
    /// assert!(win.next().is_none());
    /// ```
    pub fn new(elements: &'a [T], size: usize, step: usize) -> Self {
        assert!(size > 0);
        assert!(step > 0);
        assert!(step <= size);
        Windows { elements, start: 0, size, step }
    }


    /// The length of the iterator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use windowrs::Windows;
    ///
    /// let win = Windows::new(&[1, 2, 3, 4], 2, 1);
    /// assert_eq!(win.len(), 3);
    ///
    /// let win = Windows::new(&[1, 2, 3, 4, 5], 3, 2);
    /// assert_eq!(win.len(), 2);
    /// ```
    pub fn len(&self) -> usize {

        if self.elements.is_empty() {
            return 0
        }

        // Catches underflow because unsigned ints.
        let len = if self.elements.len() < self.size {
            0
        } else {
            self.elements.len() - self.size
        };

        let ndivs = len / self.step;
        let rem = if len % self.step == 0 {
            0
        } else {
            1
        };

        // +1 because cannot be empty at this point.
        ndivs + rem + 1
    }


    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}



impl<'a, T> Iterator for Windows<'a, T>
        where T: fmt::Debug {
    type Item = Window<&'a [T]>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.elements.is_empty() {
            return None
        }

        let size = if self.size > self.elements.len() {
            self.elements.len()
        } else {
            self.size
        };

        let ret = Window::new(
            self.start,
            self.start + size,
            &self.elements[..size],
        );

        self.start += self.step;

        if self.elements.len() > size {
            self.elements = &self.elements[self.step..];
        } else {
            self.elements = &[];
        }

        Some(ret)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let l = self.len();
        (l, Some(l))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let pos = n * self.step;
        let last = (self.len() - 1) * self.step;

        if pos > last {
            self.elements = &[];
            None
        } else {
            let size = if self.size > self.elements.len() {
                self.elements.len()
            } else {
                self.size
            };

            let nth = Window::new(
                pos,
                pos + size,
                &self.elements[pos..(pos + size)],
            );

            self.elements = &self.elements[(pos + self.step)..];
            self.start = pos + self.step;
            Some(nth)
        }
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        if self.elements.is_empty() {
            return None
        }

        let start = (self.len() - 1) * self.step;
        let rec = Window::new(start, self.elements.len(), &self.elements[start..]);
        Some(rec)
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Window<V> {
    pub start: usize,
    pub end: usize,
    pub value: V,
}


impl<V> Window<V> {

    /// Create a new window object.
    pub fn new(start: usize, end: usize, value: V) -> Self {
        assert!(start <= end);
        Window { start, end, value }
    }

    /// Converts from [`Window<V>`] to [`Window<&V>`].
    ///
    /// [`Window<V>`]: struct.Window.html
    /// [`Window<&V>`]: struct.Window.html
    pub fn as_ref(&self) -> Window<&V> {
        Window::new(self.start, self.end, &self.value)
    }

    /// Converts from [`Window<V>`] to [`Window<&mut V>`].
    ///
    /// [`Window<V>`]: struct.Window.html
    /// [`Window<&mut V>`]: struct.Window.html
    ///
    /// # Examples:
    ///
    /// ```
    /// use windowrs::Window;
    /// let mut win = Window::new(1, 10, 4.0);
    /// {
    ///     let win2 = win.as_mut();
    ///     *win2.value = 3.0;
    /// } // Mutable ref goes out of scope here
    /// assert_eq!(Window::new(1, 10, 3.0), win);
    /// ```
    pub fn as_mut(&mut self) -> Window<&mut V> {
        Window::new(self.start, self.end, &mut (*self).value)
    }

    /// Applies a function to the value in a window, leaving start and end
    /// unchanged.
    ///
    /// Examples:
    ///
    /// ```
    /// use windowrs::Window;
    /// let win = Window::new(1, 10, 3.0);
    /// let result = win.map(|x| x > 2.0);
    /// assert_eq!(result, Window::new(1, 10, true));
    /// ```
    pub fn map<U, F: FnOnce(V) -> U>(self, f: F) -> Window<U> {
        Window::new(self.start, self.end, f(self.value))
    }

    /// Applies a fuction taking the value and returning a new Window object.
    ///
    /// Examples:
    ///
    /// ```
    /// use windowrs::Window;
    /// let win = Window::new(1, 10, 3.0);
    /// let result = win.flat_map(|x| Window::new(0, 0, x));
    /// assert_eq!(Window::new(0, 0, 3.0), result);
    /// ```
    pub fn flat_map<U, F: FnOnce(V) -> Window<U>>(self, f: F) -> Window<U> {
        f(self.value)
    }
}


pub trait Windower {
    type Item;

    fn into_windows(&self, size: usize, step: usize) -> Windows<&Self::Item>;
}


#[cfg(test)]
mod test {
    use super::Windows;
    use super::Window;

    #[test]
    fn can_use_next() {
        let elems = &[1, 2, 3, 4];
        let mut win = Windows::new(elems, 2, 1);
        assert_eq!(win.next().unwrap(), Window::new(0, 2, &[1, 2][..]));
        assert_eq!(win.next().unwrap(), Window::new(1, 3, &[2, 3][..]));
        assert_eq!(win.next().unwrap(), Window::new(2, 4, &[3, 4][..]));
        assert!(win.next().is_none());

        let elems = &[1, 2, 3, 4, 5, 6, 7];
        let mut win = Windows::new(elems, 3, 1);
        assert_eq!(win.next().unwrap(), Window::new(0, 3, &[1, 2, 3][..]));
        assert_eq!(win.next().unwrap(), Window::new(1, 4, &[2, 3, 4][..]));
        assert_eq!(win.next().unwrap(), Window::new(2, 5, &[3, 4, 5][..]));
        assert_eq!(win.next().unwrap(), Window::new(3, 6, &[4, 5, 6][..]));
        assert_eq!(win.next().unwrap(), Window::new(4, 7, &[5, 6, 7][..]));
        assert!(win.next().is_none());
    }

    #[test]
    fn handles_incomplete_windows() {
        let elems = &[1, 2, 3, 4];
        let mut win = Windows::new(elems, 3, 2);
        assert_eq!(win.next().unwrap(), Window::new(0, 3, &[1, 2, 3][..]));
        assert_eq!(win.next().unwrap(), Window::new(2, 4, &[3, 4][..]));
        assert!(win.next().is_none());

        let elems = &[1, 2, 3, 4, 5, 6];
        let mut win = Windows::new(elems, 3, 2);
        assert_eq!(win.next().unwrap(), Window::new(0, 3, &[1, 2, 3][..]));
        assert_eq!(win.next().unwrap(), Window::new(2, 5, &[3, 4, 5][..]));
        assert_eq!(win.next().unwrap(), Window::new(4, 6, &[5, 6][..]));
        assert!(win.next().is_none());
    }

    #[test]
    fn handles_single_step() {
        let elems = &[1, 2, 3, 4];
        let mut win = Windows::new(elems, 4, 2);
        assert_eq!(win.next().unwrap(), Window::new(0, 4, &[1, 2, 3, 4][..]));
        assert!(win.next().is_none());

        let elems: &[u8] = b"This is";
        let mut win = Windows::new(elems, 20, 1);
        assert_eq!(
            win.next().unwrap(),
            Window::new(0, 7, b"This is" as &[u8])
        );
        assert!(win.next().is_none());
    }

    #[test]
    fn correct_len() {
        let elems = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let win = Windows::new(elems, 6, 2);
        assert_eq!(win.len(), 4);
        let win = Windows::new(elems, 7, 2);
        assert_eq!(win.len(), 4);
        let win = Windows::new(elems, 7, 3);
        assert_eq!(win.len(), 3);

        let elems = &[1, 2, 3, 4, 5, 6, 7];
        let win = Windows::new(elems, 3, 1);
        assert_eq!(win.len(), 5);
        let win = Windows::new(elems, 2, 1);
        assert_eq!(win.len(), 6);

        let elems = &[1, 2, 3, 4, 5, 6];
        let win = Windows::new(elems, 1, 1);
        assert_eq!(win.len(), 6);
        let win = Windows::new(elems, 3, 2);
        assert_eq!(win.len(), 3);
        let win = Windows::new(elems, 3, 3);
        assert_eq!(win.len(), 2);
        let win = Windows::new(elems, 4, 2);
        assert_eq!(win.len(), 2);
        let win = Windows::new(elems, 4, 3);
        assert_eq!(win.len(), 2);
        let win = Windows::new(elems, 6, 3);
        assert_eq!(win.len(), 1);
    }

    #[test]
    fn gets_last() {
        let elems = &[1, 2, 3, 4];
        let win = Windows::new(elems, 3, 2);
        assert_eq!(win.last().unwrap(), Window::new(2, 4, &[3, 4][..]));

        let elems = &[1, 2, 3, 4, 5, 6];
        let win = Windows::new(elems, 3, 2);
        assert_eq!(win.last().unwrap(), Window::new(4, 6, &[5, 6][..]));
    }

    #[test]
    fn gets_nth() {
        let elems = &[1, 2, 3, 4, 5, 6, 7];
        let mut win = Windows::new(elems, 3, 1);
        assert_eq!(win.nth(1).unwrap(), Window::new(1, 4, &[2, 3, 4][..]));
        assert_eq!(win.next().unwrap(), Window::new(2, 5, &[3, 4, 5][..]));

        let mut win = Windows::new(elems, 3, 1);
        assert_eq!(win.nth(3).unwrap(), Window::new(3, 6, &[4, 5, 6][..]));
        assert_eq!(win.next().unwrap(), Window::new(4, 7, &[5, 6, 7][..]));

        println!("last");
        let mut win = Windows::new(elems, 3, 2);
        assert_eq!(win.nth(1).unwrap(), Window::new(2, 5, &[3, 4, 5][..]));
        assert_eq!(win.next().unwrap(), Window::new(4, 7, &[5, 6, 7][..]));
        assert!(win.next().is_none());
    }

    #[test]
    fn maps_ok() {
        let elems: &[u32] = &[1, 1, 1, 2, 2, 2, 3, 3];
        let v: Vec<Window<u32>> = Windows::new(elems, 3, 1)
            .map(|w| {
                w.map(|v| v.iter().cloned().fold(0u32, u32::max))
            })
            .collect();

        assert_eq!(v[0], Window::new(0, 3, 1));
        assert_eq!(v[5], Window::new(5, 8, 3));
    }
}
