#[allow(dead_code)]
use std::alloc::{Layout, alloc, dealloc, realloc};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::{mem, ptr};

struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}

impl<T> RawVec<T> {
    fn new() -> Self {
        assert!(mem::size_of::<T>() > 0, "TODO:实现零尺寸类型的支持");
        Self {
            ptr: NonNull::dangling(),
            cap: 0,
        }
    }

    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
        let new_layout = Layout::array::<T>(new_cap).unwrap();
        let new_ptr = if self.cap == 0 {
            unsafe { alloc(new_layout) }
        } else {
            unsafe {
                let old_layout = Layout::array::<T>(self.cap).unwrap();
                realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_layout.size())
            }
        };
        if new_ptr.is_null() {
            panic!("realloc failed");
        }
        self.ptr = NonNull::new(new_ptr as *mut T).unwrap();
        self.cap = new_cap;
    }
}

impl<T> Default for RawVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap > 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

pub struct SelfVec<T> {
    // ptr: NonNull<T>,
    raw_vec: RawVec<T>,
    len: usize,
    // cap: usize,
}

impl<T> SelfVec<T> {
    pub fn new() -> Self {
        assert!(
            mem::size_of::<T>() > 0,
            "zero sized types are not supported"
        );
        Self {
            // ptr: NonNull::dangling(),
            raw_vec: RawVec::default(),
            len: 0,
            // cap: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn cap(&self) -> usize {
        self.raw_vec.cap
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.raw_vec.cap {
            self.raw_vec.grow();
        }

        unsafe {
            ptr::write(self.raw_vec.ptr.as_ptr().add(self.len), value);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let result = unsafe { ptr::read(self.raw_vec.ptr.as_ptr().add(self.len - 1)) };
        self.len -= 1;
        Some(result)
    }

    pub fn insert(&mut self, index: usize, element: T) {
        assert!(index <= self.len);
        if self.len == self.raw_vec.cap {
            self.raw_vec.grow();
        }
        unsafe {
            ptr::copy(
                self.raw_vec.ptr.as_ptr().add(index),
                self.raw_vec.ptr.as_ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.raw_vec.ptr.as_ptr().add(index), element);
        }
        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len);
        let result = unsafe { ptr::read(self.raw_vec.ptr.as_ptr().add(index)) };
        unsafe {
            ptr::copy(
                self.raw_vec.ptr.as_ptr().add(index + 1),
                self.raw_vec.ptr.as_ptr().add(index),
                self.len - index - 1,
            );
        }
        self.len -= 1;
        result
    }

    // fn grow(&mut self) {
    //     let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
    //     let new_layout = Layout::array::<T>(new_cap).unwrap();
    //     let new_ptr = if self.cap == 0 {
    //         unsafe { alloc(new_layout) }
    //     } else {
    //         let old_layout = Layout::array::<T>(self.cap).unwrap();
    //         unsafe { realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_layout.size()) }
    //     };
    //     if new_ptr.is_null() {
    //         panic!("allocation failure");
    //     }
    //     self.ptr = NonNull::new(new_ptr as *mut T).unwrap();
    //     self.cap = new_cap;
    // }
}

impl<T> Default for SelfVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for SelfVec<T> {
    fn drop(&mut self) {
        std::ptr::slice_from_raw_parts_mut(self.raw_vec.ptr.as_ptr(), self.len);
    }
}

impl<T> Deref for SelfVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.raw_vec.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for SelfVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.raw_vec.ptr.as_ptr(), self.len) }
    }
}

pub struct IntoIter<T> {
    _raw_vec: RawVec<T>,
    start: *const T,
    end: *const T,
}

// impl<T>  SelfVec<T> {
//     pub fn into_iter(mut self) -> IntoIter<T> {
//         let len = self.len;
//         let cap = self.raw_vec.cap;
//         let p = self.raw_vec.ptr.as_ptr() as *const T;
//         let raw = std::mem::take(&mut self.raw_vec);
//
//         let end = if cap == 0 { p } else { unsafe { p.add(len) } };
//         IntoIter {
//             _raw_vec: raw,
//             start: p,
//             end,
//         }
//     }
// }

impl<T> IntoIterator for SelfVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(mut self) -> Self::IntoIter {
        let len = self.len;
        let p = self.raw_vec.ptr.as_ptr() as *const T;
        // take 将raw_vec移出self，self的raw_vec变成默认空，self drop不会再释放内存
        let _raw_vec = mem::take(&mut self.raw_vec);
        // self.len 不需要置零，因为self马上就销毁，raw_vec已经被take走

        let end = if _raw_vec.cap == 0 {
            p
        } else {
            unsafe { p.add(len) }
        };

        IntoIter {
            _raw_vec,
            start: p,
            end,
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            return None;
        }
        unsafe {
            let result = ptr::read(self.start);
            self.start = self.start.add(1);
            Some(result)
        }
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        if self.start != self.end {
            while self.next().is_some() {}
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::SelfVec;
    use std::ops::{Deref, DerefMut};

    fn default_self_vec() -> SelfVec<usize> {
        let mut v = SelfVec::default();
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        v.push(5);
        v
    }

    #[test]
    fn push() {
        let mut v = SelfVec::default();
        v.push(1);
        assert_eq!(v.len(), 1);
        assert_eq!(v.cap(), 1);
        v.push(2);
        assert_eq!(v.len(), 2);
        assert_eq!(v.cap(), 2);
        v.push(3);
        assert_eq!(v.len(), 3);
        assert_eq!(v.cap(), 4);
    }

    #[test]
    fn pop() {
        let mut v = default_self_vec();
        assert_eq!(v.pop(), Some(5));
        assert_eq!(v.pop(), Some(4));
        assert_eq!(v.pop(), Some(3));
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.pop(), None);
    }

    #[test]
    fn insert() {
        let mut v = SelfVec::default();
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        v.insert(2, 6);
        assert_eq!(v.len(), 5);
        assert_eq!(v.cap(), 8);
        assert_eq!(v.pop(), Some(4));
        assert_eq!(v.pop(), Some(3));
        assert_eq!(v.pop(), Some(6));
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.len(), 0);
        assert_eq!(v.cap(), 8);
    }

    #[test]
    fn remove() {
        let mut v = SelfVec::default();
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        let removed = v.remove(2);
        assert_eq!(removed, 3);
        assert_eq!(v.len(), 3);
        assert_eq!(v.cap(), 4);
        assert_eq!(v.pop(), Some(4));
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.pop(), None);
    }

    #[test]
    fn deref() {
        let v = default_self_vec();
        let v_deref = v.deref();
        assert_eq!(v_deref.len(), 5);
        let mut iter = v_deref.iter();
        assert_eq!(iter.len(), 5);
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(v_deref.first(), Some(&1));
        assert_eq!(v_deref.last(), Some(&5));
    }

    #[test]
    fn deref_mut() {
        let mut v = default_self_vec();
        let v_deref = v.deref_mut();
        assert_eq!(v_deref.len(), 5);
        let iter_mut = v_deref.iter_mut();
        assert_eq!(iter_mut.len(), 5);
        for v in iter_mut {
            *v += 1;
        }
        assert_eq!(v_deref.first(), Some(&2));
        assert_eq!(v_deref.last(), Some(&6));
    }

    #[test]
    fn into_iter() {
        let v = default_self_vec();
        let mut v_into_iter = v.into_iter();
        assert_eq!(v_into_iter.next(), Some(1));
        assert_eq!(v_into_iter.next(), Some(2));
        assert_eq!(v_into_iter.next(), Some(3));
        assert_eq!(v_into_iter.next(), Some(4));
        assert_eq!(v_into_iter.next(), Some(5));
        assert_eq!(v_into_iter.next(), None);

        let v = default_self_vec();
        let mut v_into_iter = v.into_iter();
        assert_eq!(v_into_iter.next_back(), Some(5));
        assert_eq!(v_into_iter.next_back(), Some(4));
        assert_eq!(v_into_iter.next_back(), Some(3));
        assert_eq!(v_into_iter.next_back(), Some(2));
        assert_eq!(v_into_iter.next_back(), Some(1));
        assert_eq!(v_into_iter.next_back(), None);
    }
}
