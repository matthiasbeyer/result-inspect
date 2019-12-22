//! This crate adds the missing Result::inspect function via an extension trait
//!

pub trait ResultInspect<F, T>
    where F: FnOnce(&T),
          T: Sized
{
    fn inspect(self, f: F) -> Self;
}

pub trait ResultInspectRef<F, T>
    where F: FnOnce(&T),
          T: Sized
{
    fn inspect(&self, f: F);
}

impl<F, T, E> ResultInspect<F, T> for Result<T, E>
    where F: FnOnce(&T),
          T: Sized
{
    fn inspect(self, f: F) -> Self {
        if let Ok(ref o) = self.as_ref() {
            (f)(&o);
        }

        self
    }
}

impl<F, T, E> ResultInspectRef<F, T> for Result<T, E>
    where F: FnOnce(&T),
          T: Sized
{
    fn inspect(&self, f: F) {
        if let Ok(ref o) = self {
            (f)(&o);
        }
    }
}
