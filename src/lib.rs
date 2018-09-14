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
}


impl<'a, T> Windows<'a, T> {

    /// Create a new windows object.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use windowers::Windows;
    ///
    /// let mut win = Windows::new(&[1, 2, 3, 4], 2, 1);
    /// assert_eq!(win.next().unwrap(), &[1, 2]);
    /// assert_eq!(win.next().unwrap(), &[2, 3]);
    /// assert_eq!(win.next().unwrap(), &[3, 4]);
    /// assert!(win.next().is_none());
    /// ```
    pub fn new(elements: &'a [T], size: usize, step: usize) -> Self {
        assert!(size > 0);
        assert!(step > 0);
        assert!(step <= size);
        Windows { elements: elements, size: size, step: step }
    }


    /// The length of the iterator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use windowers::Windows;
    ///
    /// let win = Windows::new(&[1, 2, 3, 4], 2, 1);
    /// assert_eq!(win.len(), 3);
    ///
    /// let win = Windows::new(&[1, 2, 3, 4, 5], 3, 2);
    /// assert_eq!(win.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        let len = self.elements.len() - self.size;

        let ndivs = len / self.step;
        let rem = if len % self.step == 0 {
            0
        } else {
            1
        };
        let size = ndivs + rem + 1;
        size
    }
}



impl<'a, T> Iterator for Windows<'a, T>
        where T: fmt::Debug {
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.elements.len() == 0 {
            return None
        }

        let size = if self.size > self.elements.len() {
            self.elements.len()
        } else {
            self.size
        };

        let ret = &self.elements[..size];

        if (self.elements.len() - size) > 0 {
            self.elements = &self.elements[self.step..];
        } else {
            self.elements = &[];
        }
        println!("{:?}", ret);
        Some(ret)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let l = self.len();
        (l, Some(l))
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.size.overflowing_add(n);
        if end > self.elements.len() || overflow {
            self.elements = &[];
            None
        } else {
            let nth = &self.elements[n..end];
            self.elements = &self.elements[(n + 1)..];
            Some(nth)
        }
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        if self.elements.len() == 0 {
            return None
        }

        let start = (self.len() - 1) * self.step;
        Some(&self.elements[start..])
    }
}


//pub trait Windower : for<'a> From<&'a T> {
//    fn into_windows(&self, size: usize, step: usize) -> Windows<&'a T>;
//}
//
//
//impl<'a, T> Windower for &'a, [T] {
//    fn into_windows(&self, size: usize, step: usize) -> Windows<&'a, [T]> {
//        Windows::new(self, size, step)
//    }
//}



#[cfg(test)]
mod test {
    use super::Windows;

    #[test]
    fn can_use_next() {
        let elems = &[1, 2, 3, 4];
        let mut win = Windows::new(elems, 2, 1);
        assert_eq!(win.next().unwrap(), &[1, 2]);
        assert_eq!(win.next().unwrap(), &[2, 3]);
        assert_eq!(win.next().unwrap(), &[3, 4]);
        assert!(win.next().is_none());

        let elems = &[1, 2, 3, 4, 5, 6, 7];
        let mut win = Windows::new(elems, 3, 1);
        assert_eq!(win.next().unwrap(), &[1, 2, 3]);
        assert_eq!(win.next().unwrap(), &[2, 3, 4]);
        assert_eq!(win.next().unwrap(), &[3, 4, 5]);
        assert_eq!(win.next().unwrap(), &[4, 5, 6]);
        assert_eq!(win.next().unwrap(), &[5, 6, 7]);
        assert!(win.next().is_none());
    }

    #[test]
    fn handles_incomplete_windows() {
        let elems = &[1, 2, 3, 4];
        let mut win = Windows::new(elems, 3, 2);
        assert_eq!(win.next().unwrap(), &[1, 2, 3]);
        assert_eq!(win.next().unwrap(), &[3, 4]);
        assert!(win.next().is_none());

        let elems = &[1, 2, 3, 4, 5, 6];
        let mut win = Windows::new(elems, 3, 2);
        assert_eq!(win.next().unwrap(), &[1, 2, 3]);
        assert_eq!(win.next().unwrap(), &[3, 4, 5]);
        assert_eq!(win.next().unwrap(), &[5, 6]);
        assert!(win.next().is_none());
    }

    #[test]
    fn handles_single_step() {
        let elems = &[1, 2, 3, 4];
        let mut win = Windows::new(elems, 4, 2);
        assert_eq!(win.next().unwrap(), &[1, 2, 3, 4]);
        assert!(win.next().is_none());

        let elems: &[u8] = b"This is";
        let mut win = Windows::new(elems, 7, 1);
        assert_eq!(win.next().unwrap(), (b"This is" as &[u8]));
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
        assert_eq!(win.last().unwrap(), &[3, 4]);

        let elems = &[1, 2, 3, 4, 5, 6];
        let win = Windows::new(elems, 3, 2);
        assert_eq!(win.last().unwrap(), &[5, 6]);
    }
}
