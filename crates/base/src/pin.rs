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


#[cfg(test)]
mod tests {
    use crate::pin::SelfRef;

    #[test]
    fn normal() {
        let mut sr1 = SelfRef::new("test1");
        let mut sr2 = SelfRef::new("test2");

        assert_eq!(sr1.as_ref().a(), sr1.as_ref().b());
        assert_eq!(sr1.as_ref().b(), "test1");
        assert_eq!(sr2.as_ref().a(), sr2.as_ref().b());
        assert_eq!(sr2.as_ref().b(), "test2");
        std::mem::swap(&mut sr1, &mut sr2);
        assert_eq!(sr1.as_ref().a(), sr1.as_ref().b());
        assert_eq!(sr1.as_ref().b(), "test2");
        assert_eq!(sr2.as_ref().a(), sr2.as_ref().b());
        assert_eq!(sr2.as_ref().b(), "test1");
    }
}