//! This crate adds the missing Result::inspect function via an extension trait
//!

/// Extension trait for adding Result::inspect
pub trait ResultInspect<F, T>
where
    F: FnOnce(&T),
    T: Sized,
{
    /// Call `f` on T if the Result is a Ok(T), or do nothing if the Result is an Err
    fn inspect(self, f: F) -> Self;
}

pub trait ResultInspectRef<F, T>
where
    F: FnOnce(&T),
    T: Sized,
{
    fn inspect(&self, f: F);
}

impl<F, T, E> ResultInspect<F, T> for Result<T, E>
where
    F: FnOnce(&T),
    T: Sized,
{
    /// Call `f` on T if the Result is a Ok(T), or do nothing if the Result is an Err
    fn inspect(self, f: F) -> Self {
        if let Ok(o) = self.as_ref() {
            (f)(o);
        }

        self
    }
}

impl<F, T, E> ResultInspectRef<F, T> for Result<T, E>
where
    F: FnOnce(&T),
    T: Sized,
{
    fn inspect(&self, f: F) {
        if let Ok(ref o) = self {
            (f)(o);
        }
    }
}

/// Extension trait for adding Result::inspect_err
pub trait ResultInspectErr<F, E>
where
    F: FnOnce(&E),
    E: Sized,
{
    /// Call `f` on T if the Result is a Err(E), or do nothing if the Result is an Ok
    fn inspect_err(self, f: F) -> Self;
}

pub trait ResultInspectErrRef<F, E>
where
    F: FnOnce(&E),
    E: Sized,
{
    fn inspect_err(&self, f: F);
}

impl<F, T, E> ResultInspectErr<F, E> for Result<T, E>
where
    F: FnOnce(&E),
    E: Sized,
{
    /// Call `f` on T if the Result is a Err(E), or do nothing if the Result is an Ok
    fn inspect_err(self, f: F) -> Self {
        if let Err(e) = self.as_ref() {
            (f)(e);
        }

        self
    }
}

impl<F, T, E> ResultInspectErrRef<F, E> for Result<T, E>
where
    F: FnOnce(&E),
    E: Sized,
{
    fn inspect_err(&self, f: F) {
        if let Err(ref e) = self {
            (f)(e);
        }
    }
}
