#[macro_export]
macro_rules! guarded_unwrap {
    (@inner $expr:expr, $none_case:expr) => {
        match guarded_unwrap::GuardedUnwrap::guarded_unwrap_inner($expr) {
            Some(value) => value,
            None => $none_case,
        }
    };

    ($expr:expr, return $ret:expr) => {
        guarded_unwrap!(@inner $expr, { return $ret; })
    };

    ($expr:expr, return) => {
        guarded_unwrap!(@inner $expr, { return; })
    };

    ($expr:expr, break $ret:expr) => {
        guarded_unwrap!(@inner $expr, { break $ret; })
    };

    ($expr:expr, break) => {
        guarded_unwrap!(@inner $expr, { break; })
    };

    ($expr:expr, continue $ret:expr) => {
        guarded_unwrap!(@inner $expr, { continue $ret; })
    };

    ($expr:expr, continue) => {
        guarded_unwrap!(@inner $expr, { continue; })
    };
}

pub trait GuardedUnwrap<T> {
    fn guarded_unwrap_inner(self) -> Option<T>;
}

impl<T> GuardedUnwrap<T> for Option<T> {
    fn guarded_unwrap_inner(self) -> Option<T> {
        self
    }
}

impl<T, E> GuardedUnwrap<T> for Result<T, E> {
    fn guarded_unwrap_inner(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(_) => None
        }
    }
}