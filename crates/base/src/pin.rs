use std::marker::PhantomPinned;
use std::pin::Pin;

#[derive(Debug)]
pub struct SelfRef {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl SelfRef {
    pub fn new(s: &str) -> Pin<Box<Self>> {
        let s = SelfRef {
            a: s.to_string(),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(s);
        let self_ref: *const String = &boxed.as_ref().a;
        unsafe {
            boxed.as_mut().get_unchecked_mut().b = self_ref;
        }

        boxed
    }

    pub fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    pub fn b(self: Pin<&Self>) -> &String {
        unsafe { &*(self.b) }
    }
}
