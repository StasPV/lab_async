use std::{marker::PhantomPinned, pin::Pin};

#[derive(Debug)]
pub struct Test{
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}
impl Test{
    pub fn new(txt: &str)->Pin<Box<Self>>{
        let t  = Test{
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.a;
        unsafe {boxed.as_mut().get_unchecked_mut().b = self_ptr};
        boxed
    }

    pub fn init(self: Pin<&mut Self>){
        let self_ptr: *const String = &self.a;
        let this = unsafe{ self.get_unchecked_mut()};
        this.b = self_ptr;
    }

    pub fn a(self: Pin<&Self>)->&str{
        &self.get_ref().a
    }

    pub fn b(self: Pin<&Self>)->&str{
        assert!(!self.b.is_null());
        unsafe{ &*(self.b)}
    }
}